//! Detecção de formato pelo conteúdo (RF02, RN14, Apêndice D).
//!
//! O formato real de um arquivo é decidido pelos primeiros bytes ("magic
//! bytes"), nunca pela extensão. Aqui estão os 8 formatos de imagem; PDF e ZIP
//! entram nos Marcos 4 e 5.

use crate::error::CoreError;

/// Quantos bytes do início do arquivo a detecção olha nos formatos binários
/// (RF02: até 4 KB).
pub const HEADER_LEN: usize = 4096;

/// O SVG é texto e pode começar com comentários e declarações longas; para ele
/// a detecção olha até 64 KB (RF02). É o tamanho que a interface precisa
/// entregar a `detect`.
pub const SVG_HEADER_LEN: usize = 64 * 1024;

/// Formatos de imagem que o núcleo sabe ler. Todos, exceto SVG, também são
/// formatos de saída (§5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ImageFormat {
    Png,
    Jpeg,
    WebP,
    Gif,
    Bmp,
    Tiff,
    Ico,
    Svg,
}

impl ImageFormat {
    /// Todos os formatos, na ordem do §5.1.
    pub const ALL: [ImageFormat; 8] = [
        ImageFormat::Png,
        ImageFormat::Jpeg,
        ImageFormat::WebP,
        ImageFormat::Gif,
        ImageFormat::Bmp,
        ImageFormat::Tiff,
        ImageFormat::Ico,
        ImageFormat::Svg,
    ];

    /// Nome curto exibido na interface (`PNG → JPG`).
    pub fn label(self) -> &'static str {
        match self {
            ImageFormat::Png => "PNG",
            ImageFormat::Jpeg => "JPG",
            ImageFormat::WebP => "WebP",
            ImageFormat::Gif => "GIF",
            ImageFormat::Bmp => "BMP",
            ImageFormat::Tiff => "TIFF",
            ImageFormat::Ico => "ICO",
            ImageFormat::Svg => "SVG",
        }
    }

    /// Extensão padrão da saída, sempre minúscula (RN15).
    pub fn extension(self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
            ImageFormat::WebP => "webp",
            ImageFormat::Gif => "gif",
            ImageFormat::Bmp => "bmp",
            ImageFormat::Tiff => "tiff",
            ImageFormat::Ico => "ico",
            ImageFormat::Svg => "svg",
        }
    }

    /// Extensões aceitas na entrada (§5.1), usadas só para avisar quando a
    /// extensão não bate com o conteúdo (RN14).
    pub fn input_extensions(self) -> &'static [&'static str] {
        match self {
            ImageFormat::Png => &["png"],
            ImageFormat::Jpeg => &["jpg", "jpeg", "jpe", "jfif"],
            ImageFormat::WebP => &["webp"],
            ImageFormat::Gif => &["gif"],
            ImageFormat::Bmp => &["bmp", "dib"],
            ImageFormat::Tiff => &["tif", "tiff"],
            ImageFormat::Ico => &["ico"],
            ImageFormat::Svg => &["svg"],
        }
    }

    /// SVG só entra; não existe saída em SVG (§5.1).
    pub fn can_be_output(self) -> bool {
        self != ImageFormat::Svg
    }
}

const PNG_MAGIC: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
const JPEG_MAGIC: &[u8] = &[0xFF, 0xD8, 0xFF];

/// Identifica o formato a partir do início do arquivo.
///
/// Retorna `E-103` para arquivo vazio e `E-102` quando nenhuma assinatura
/// conhecida aparece.
pub fn detect(header: &[u8]) -> Result<ImageFormat, CoreError> {
    if header.is_empty() {
        return Err(CoreError::EmptyFile);
    }
    if header.starts_with(PNG_MAGIC) {
        return Ok(ImageFormat::Png);
    }
    if header.starts_with(JPEG_MAGIC) {
        return Ok(ImageFormat::Jpeg);
    }
    if header.starts_with(b"GIF87a") || header.starts_with(b"GIF89a") {
        return Ok(ImageFormat::Gif);
    }
    // RIFF....WEBP (os 4 bytes do meio são o tamanho).
    if header.len() >= 12 && &header[0..4] == b"RIFF" && &header[8..12] == b"WEBP" {
        return Ok(ImageFormat::WebP);
    }
    // TIFF clássico (42) e BigTIFF (43), little e big endian.
    if header.starts_with(&[0x49, 0x49, 0x2A, 0x00])
        || header.starts_with(&[0x4D, 0x4D, 0x00, 0x2A])
        || header.starts_with(&[0x49, 0x49, 0x2B, 0x00])
        || header.starts_with(&[0x4D, 0x4D, 0x00, 0x2B])
    {
        return Ok(ImageFormat::Tiff);
    }
    // ICO: 00 00 01 00 e pelo menos uma imagem. CUR (00 00 02 00) é recusado.
    if header.starts_with(&[0x00, 0x00, 0x01, 0x00])
        && header.len() >= 6
        && u16::from_le_bytes([header[4], header[5]]) > 0
    {
        return Ok(ImageFormat::Ico);
    }
    // "BM" são só 2 bytes e qualquer texto pode começar assim; por isso o
    // tamanho do cabeçalho de imagem (offset 14) também precisa ser um dos
    // valores que existem de fato.
    if header.starts_with(b"BM") && is_bmp(header) {
        return Ok(ImageFormat::Bmp);
    }
    if looks_like_svg(header) {
        return Ok(ImageFormat::Svg);
    }
    Err(CoreError::UnknownFormat)
}

fn is_bmp(header: &[u8]) -> bool {
    if header.len() < 18 {
        return false;
    }
    let dib = u32::from_le_bytes([header[14], header[15], header[16], header[17]]);
    matches!(dib, 12 | 40 | 52 | 56 | 64 | 108 | 124)
}

/// SVG é texto: depois de BOM, espaços, declaração XML, comentários e DOCTYPE,
/// o primeiro elemento precisa ser `<svg` (RF02, Apêndice D). Se o arquivo
/// não for XML válido, o erro aparece depois, na leitura (E-105).
fn looks_like_svg(header: &[u8]) -> bool {
    let mut rest = header.strip_prefix(&[0xEF, 0xBB, 0xBF]).unwrap_or(header);
    loop {
        rest = skip_ascii_space(rest);
        if let Some(after) = rest.strip_prefix(b"<?") {
            match find(after, b"?>") {
                Some(i) => rest = &after[i + 2..],
                None => return false,
            }
        } else if let Some(after) = rest.strip_prefix(b"<!--") {
            match find(after, b"-->") {
                Some(i) => rest = &after[i + 3..],
                None => return false,
            }
        } else if rest.len() >= 9 && rest[..9].eq_ignore_ascii_case(b"<!DOCTYPE") {
            match doctype_end(rest) {
                Some(i) => rest = &rest[i..],
                None => return false,
            }
        } else {
            break;
        }
    }
    // "<svg" seguido de espaço, ">" ou "/" (e não "<svgfoo").
    rest.starts_with(b"<svg")
        && rest
            .get(4)
            .is_some_and(|c| c.is_ascii_whitespace() || *c == b'>' || *c == b'/')
}

/// Fim de um DOCTYPE, que pode ter um subconjunto interno entre colchetes.
fn doctype_end(s: &[u8]) -> Option<usize> {
    let mut depth = 0i32;
    for (i, b) in s.iter().enumerate() {
        match b {
            b'[' => depth += 1,
            b']' => depth -= 1,
            b'>' if depth <= 0 => return Some(i + 1),
            _ => {}
        }
    }
    None
}

fn skip_ascii_space(s: &[u8]) -> &[u8] {
    let n = s.iter().take_while(|b| b.is_ascii_whitespace()).count();
    &s[n..]
}

fn find(haystack: &[u8], needle: &[u8]) -> Option<usize> {
    haystack.windows(needle.len()).position(|w| w == needle)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconhece_os_formatos_binarios() {
        assert_eq!(detect(PNG_MAGIC), Ok(ImageFormat::Png));
        assert_eq!(detect(&[0xFF, 0xD8, 0xFF, 0xE0]), Ok(ImageFormat::Jpeg));
        assert_eq!(detect(b"GIF89a\x01\x00"), Ok(ImageFormat::Gif));
        assert_eq!(detect(b"GIF87a\x01\x00"), Ok(ImageFormat::Gif));
        assert_eq!(detect(b"RIFF\x24\0\0\0WEBPVP8 "), Ok(ImageFormat::WebP));
        assert_eq!(
            detect(&[0x49, 0x49, 0x2A, 0x00, 8, 0]),
            Ok(ImageFormat::Tiff)
        );
        assert_eq!(
            detect(&[0x4D, 0x4D, 0x00, 0x2A, 0, 8]),
            Ok(ImageFormat::Tiff)
        );
        assert_eq!(
            detect(&[0x49, 0x49, 0x2B, 0x00, 8, 0]),
            Ok(ImageFormat::Tiff)
        );
        assert_eq!(detect(&[0, 0, 1, 0, 1, 0, 16, 16]), Ok(ImageFormat::Ico));
    }

    #[test]
    fn bmp_exige_cabecalho_de_imagem_valido() {
        let mut bmp = vec![b'B', b'M'];
        bmp.extend_from_slice(&[0; 12]);
        bmp.extend_from_slice(&40u32.to_le_bytes());
        assert_eq!(detect(&bmp), Ok(ImageFormat::Bmp));
        // Texto que só começa com "BM" não é BMP.
        assert_eq!(
            detect(b"BM e um texto qualquer, nao uma imagem"),
            Err(CoreError::UnknownFormat)
        );
    }

    #[test]
    fn ico_recusa_cur_e_arquivo_sem_imagens() {
        assert_eq!(detect(&[0, 0, 2, 0, 1, 0]), Err(CoreError::UnknownFormat));
        assert_eq!(detect(&[0, 0, 1, 0, 0, 0]), Err(CoreError::UnknownFormat));
    }

    #[test]
    fn svg_depois_de_bom_declaracao_e_comentarios() {
        assert_eq!(detect(b"<svg xmlns='x'/>"), Ok(ImageFormat::Svg));
        assert_eq!(
            detect(b"\xEF\xBB\xBF  \n<?xml version='1.0'?>\n<!-- c -->\n<svg width='1'>"),
            Ok(ImageFormat::Svg)
        );
        assert_eq!(
            detect(b"<!DOCTYPE svg PUBLIC \"-//W3C//DTD SVG 1.1//EN\" \"x\" [ <!ENTITY a 'b'> ]>\n<svg>"),
            Ok(ImageFormat::Svg)
        );
        // Só <svg dentro de um texto qualquer, ou outro elemento, não conta.
        assert_eq!(
            detect(b"<html><svg></svg></html>"),
            Err(CoreError::UnknownFormat)
        );
        assert_eq!(detect(b"<svgfoo>"), Err(CoreError::UnknownFormat));
        assert_eq!(detect(b"<!-- sem fim <svg>"), Err(CoreError::UnknownFormat));
    }

    #[test]
    fn vazio_e_desconhecido() {
        assert_eq!(detect(&[]), Err(CoreError::EmptyFile));
        assert_eq!(detect(b"%PDF-1.7"), Err(CoreError::UnknownFormat));
        // Assinatura cortada no meio não conta.
        assert_eq!(detect(&PNG_MAGIC[..4]), Err(CoreError::UnknownFormat));
    }

    #[test]
    fn extensoes_de_saida_sao_minusculas() {
        for f in ImageFormat::ALL {
            assert_eq!(f.extension(), f.extension().to_lowercase());
        }
    }
}
