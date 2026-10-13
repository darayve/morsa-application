//! SVG → pixels (§5.1, §5.3, RF06).
//!
//! O SVG é um programa de desenho em texto, e arquivos de fontes não confiáveis
//! podem pedir coisas perigosas: carregar uma imagem de um endereço da internet
//! ou de um caminho do disco ("C:\segredo.png"). O Morsa nunca faz isso
//! (RNF10, SEG): só imagens embutidas no próprio arquivo (`data:`) são lidas.
//! Todo o resto é ignorado e vira o aviso E-110.

use std::sync::Arc;

use resvg::tiny_skia::{Pixmap, Transform};
use resvg::usvg::{self, fontdb};

use super::options::{MAX_SVG_SIDE, SvgSize};
use super::raster::{Raster, Samples};
use crate::error::CoreError;
use crate::limits;
use crate::warning::Warning;

/// Fonte empacotada no app (OFL-1.1). Sem fontes do sistema, o texto do SVG
/// sai igual no desktop e na web, e o resultado é reproduzível nos testes.
const FONT: &[u8] = include_bytes!(
    "../../../app/assets/fonts/ibm-plex-sans-condensed/IBMPlexSansCondensed-Medium.ttf"
);
const FONT_FAMILY: &str = "IBM Plex Sans Condensed";

/// Largura usada quando o SVG não declara `width` e `height` (RF06).
const DEFAULT_WIDTH: f32 = 1024.0;

pub fn render(input: &[u8], size: SvgSize) -> Result<(Raster, Vec<Warning>), CoreError> {
    let text = String::from_utf8_lossy(input);
    let mut warnings = Vec::new();
    if has_external_references(&text) {
        warnings.push(Warning::SvgExternalResources);
    }

    let options = options();
    let tree = usvg::Tree::from_data(input, &options).map_err(|e| CoreError::Corrupted {
        detail: e.to_string(),
    })?;

    let declared = declares_size(&text);
    let (tw, th) = target_size(tree.size().width(), tree.size().height(), declared, size);
    let (width, height) = (tw.round().max(1.0) as u32, th.round().max(1.0) as u32);
    if width > MAX_SVG_SIDE || height > MAX_SVG_SIDE {
        return Err(CoreError::ImageTooLarge { width, height });
    }
    limits::check_dimensions(width, height)?;

    let mut pixmap =
        Pixmap::new(width, height).ok_or(CoreError::ImageTooLarge { width, height })?;
    // Escala exata em cada eixo, para o resultado ocupar o pixmap inteiro
    // mesmo depois do arredondamento do tamanho.
    let transform = Transform::from_scale(
        width as f32 / tree.size().width(),
        height as f32 / tree.size().height(),
    );
    resvg::render(&tree, transform, &mut pixmap.as_mut());

    // O pixmap guarda cor pré-multiplicada pelo alfa; o resto do núcleo usa
    // cor "reta".
    let mut rgba = Vec::with_capacity(pixmap.pixels().len() * 4);
    for p in pixmap.pixels() {
        let c = p.demultiply();
        rgba.extend_from_slice(&[c.red(), c.green(), c.blue(), c.alpha()]);
    }
    Ok((
        Raster::from_samples(width, height, Samples::U8(rgba)),
        warnings,
    ))
}

fn options() -> usvg::Options<'static> {
    let mut db = fontdb::Database::new();
    db.load_font_data(FONT.to_vec());
    db.set_serif_family(FONT_FAMILY);
    db.set_sans_serif_family(FONT_FAMILY);
    db.set_cursive_family(FONT_FAMILY);
    db.set_fantasy_family(FONT_FAMILY);
    db.set_monospace_family(FONT_FAMILY);

    let mut options = usvg::Options {
        fontdb: Arc::new(db),
        font_family: FONT_FAMILY.to_string(),
        ..usvg::Options::default()
    };
    // Só imagens embutidas (`data:`) são lidas; caminhos e endereços, nunca.
    // O resolvedor padrão leria arquivos do disco.
    options.image_href_resolver.resolve_string = Box::new(|_href, _opts| None);
    options
}

/// Tamanho final em px (largura, altura), a partir do tamanho que o `usvg`
/// interpretou (`sw` × `sh`).
fn target_size(sw: f32, sh: f32, declared: bool, size: SvgSize) -> (f32, f32) {
    // Tamanho-base: o declarado, ou 1024 px de largura se não houver (RF06).
    let (bw, bh) = if declared || sw <= 0.0 {
        (sw, sh)
    } else {
        (DEFAULT_WIDTH, sh * DEFAULT_WIDTH / sw)
    };
    let (tw, th) = match size {
        SvgSize::Declared => (bw, bh),
        SvgSize::ScalePercent(p) => (bw * p as f32 / 100.0, bh * p as f32 / 100.0),
        SvgSize::Pixels {
            width: Some(w),
            height: None,
        } => (w as f32, bh * w as f32 / bw),
        SvgSize::Pixels {
            width: None,
            height: Some(h),
        } => (bw * h as f32 / bh, h as f32),
        SvgSize::Pixels {
            width: Some(w),
            height: Some(h),
        } => {
            // Cabe na caixa, mantendo a proporção.
            let k = (w as f32 / bw).min(h as f32 / bh);
            (bw * k, bh * k)
        }
        SvgSize::Pixels {
            width: None,
            height: None,
        } => (bw, bh),
    };
    (tw, th)
}

/// O elemento raiz declara `width` e `height`?
fn declares_size(svg: &str) -> bool {
    let Some(start) = svg.find("<svg") else {
        return false;
    };
    // Fim da tag de abertura, ignorando ">" dentro de aspas.
    let mut quote = None;
    let mut end = svg.len();
    for (i, c) in svg[start..].char_indices() {
        match (quote, c) {
            (None, '"' | '\'') => quote = Some(c),
            (Some(q), c) if c == q => quote = None,
            (None, '>') => {
                end = start + i;
                break;
            }
            _ => {}
        }
    }
    let tag = &svg[start..end];
    has_attribute(tag, "width") && has_attribute(tag, "height")
}

fn has_attribute(tag: &str, name: &str) -> bool {
    tag.match_indices(name).any(|(i, _)| {
        let before = tag[..i].chars().next_back();
        let after = tag[i + name.len()..].trim_start();
        before.is_some_and(|c| c.is_whitespace()) && after.starts_with('=')
    })
}

/// O SVG aponta para algo fora dele? `href` e `url(...)` que não sejam
/// âncoras internas (`#id`) nem dados embutidos (`data:`), e `@import`.
pub fn has_external_references(svg: &str) -> bool {
    let is_external = |value: &str| {
        let v = value.trim().trim_matches(|c| c == '"' || c == '\'').trim();
        !v.is_empty() && !v.starts_with('#') && !v.to_ascii_lowercase().starts_with("data:")
    };

    if svg.contains("@import") {
        return true;
    }
    // href="..." e xlink:href='...'
    for (i, _) in svg.match_indices("href") {
        let rest = svg[i + 4..].trim_start();
        let Some(rest) = rest.strip_prefix('=') else {
            continue;
        };
        let rest = rest.trim_start();
        if let Some(q) = rest.chars().next().filter(|c| *c == '"' || *c == '\'') {
            let body = &rest[1..];
            if let Some(end) = body.find(q)
                && is_external(&body[..end])
            {
                return true;
            }
        }
    }
    // url(...) em atributos e estilos.
    for (i, _) in svg.match_indices("url(") {
        let body = &svg[i + 4..];
        if let Some(end) = body.find(')')
            && is_external(&body[..end])
        {
            return true;
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn referencias_externas() {
        assert!(has_external_references(
            r#"<image href="https://x.com/a.png"/>"#
        ));
        assert!(has_external_references(
            r#"<image xlink:href='C:\segredo.png'/>"#
        ));
        assert!(has_external_references(
            r#"<rect style="fill:url(http://x/y)"/>"#
        ));
        assert!(has_external_references(
            "<style>@import url(a.css);</style>"
        ));
        // Âncoras internas e dados embutidos são legítimos.
        assert!(!has_external_references(
            r##"<use href="#a"/><rect fill="url(#g)"/>"##
        ));
        assert!(!has_external_references(
            r#"<image href="data:image/png;base64,AAAA"/>"#
        ));
        assert!(!has_external_references("<svg><rect/></svg>"));
    }

    #[test]
    fn tamanho_declarado() {
        assert!(declares_size(
            r#"<svg width="10" height="20" viewBox="0 0 1 1">"#
        ));
        assert!(declares_size(
            "<?xml version='1.0'?><svg\n height='5'\n width='6'>"
        ));
        assert!(!declares_size(r#"<svg viewBox="0 0 10 20">"#));
        assert!(!declares_size(r#"<svg width="10">"#));
        // "stroke-width" não é o atributo width.
        assert!(!declares_size(r#"<svg stroke-width="1" height="2">"#));
    }

    #[test]
    fn calculo_do_tamanho() {
        let t = |declared, size| {
            let (w, h) = target_size(200.0, 100.0, declared, size);
            (w.round() as u32, h.round() as u32)
        };
        assert_eq!(t(true, SvgSize::Declared), (200, 100));
        assert_eq!(t(false, SvgSize::Declared), (1024, 512));
        assert_eq!(t(true, SvgSize::ScalePercent(200)), (400, 200));
        assert_eq!(
            t(
                true,
                SvgSize::Pixels {
                    width: Some(50),
                    height: None
                }
            ),
            (50, 25)
        );
        assert_eq!(
            t(
                true,
                SvgSize::Pixels {
                    width: None,
                    height: Some(300)
                }
            ),
            (600, 300)
        );
        // Caixa 100×100 para um 2:1: cabe pela largura.
        assert_eq!(
            t(
                true,
                SvgSize::Pixels {
                    width: Some(100),
                    height: Some(100)
                }
            ),
            (100, 50)
        );
    }

    #[test]
    fn renderiza_formas_e_texto_com_a_fonte_empacotada() {
        let svg = br##"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="32">
            <rect width="32" height="32" fill="#ff0000"/>
            <text x="34" y="24" font-size="24" font-family="Arial" fill="#000">M</text>
        </svg>"##;
        let (r, w) = render(svg, SvgSize::Declared).unwrap();
        assert!(w.is_empty());
        assert_eq!((r.width, r.height), (64, 32));
        let Samples::U8(px) = &r.samples else {
            panic!()
        };
        let at = |x: usize, y: usize| &px[(y * 64 + x) * 4..(y * 64 + x) * 4 + 4];
        assert_eq!(at(10, 10), &[255, 0, 0, 255], "retângulo vermelho");
        assert_eq!(at(60, 2)[3], 0, "fundo transparente");
        // Algum pixel escuro onde o "M" foi desenhado: a fonte funcionou.
        let ink = (34..64).any(|x| (0..32).any(|y| at(x, y)[3] > 200));
        assert!(ink, "o texto foi renderizado");
        assert!(r.has_alpha);
    }

    #[test]
    fn imagem_do_disco_nunca_e_lida() {
        // Mesmo que o arquivo exista, o caminho é ignorado e vira aviso.
        let dir = std::env::temp_dir().join("morsa-svg-test");
        std::fs::create_dir_all(&dir).unwrap();
        let png = dir.join("segredo.png");
        std::fs::write(
            &png,
            include_bytes!("../../../corpus/image/png/valido-rgba.png"),
        )
        .unwrap();
        let svg = format!(
            r#"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="64" height="48"><image xlink:href="{}" width="64" height="48"/></svg>"#,
            png.display()
        );
        let (r, w) = render(svg.as_bytes(), SvgSize::Declared).unwrap();
        assert_eq!(w, vec![Warning::SvgExternalResources]);
        let Samples::U8(px) = &r.samples else {
            panic!()
        };
        assert!(px.chunks(4).all(|p| p[3] == 0), "nada foi desenhado");
        let _ = std::fs::remove_dir_all(&dir);
    }

    #[test]
    fn svg_invalido_e_corrompido_e_tamanho_enorme_e_recusado() {
        assert!(matches!(
            render(b"<svg><rect", SvgSize::Declared),
            Err(CoreError::Corrupted { .. })
        ));
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="100" height="100"/>"#;
        assert!(render(svg, SvgSize::ScalePercent(2000)).is_ok());
        let svg = br#"<svg xmlns="http://www.w3.org/2000/svg" width="10000" height="10000"/>"#;
        assert!(matches!(
            render(svg, SvgSize::ScalePercent(2000)),
            Err(CoreError::ImageTooLarge { .. })
        ));
    }
}
