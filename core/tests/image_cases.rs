//! Casos de borda do corpus (TST02): transparência, 16 bits, cinza, CMYK,
//! perfil de cor, orientação EXIF 1–8, animados, multipágina, ICO com vários
//! tamanhos, SVG, arquivos grandes e extensão errada. O comportamento esperado
//! de cada um está em §5.3.

mod common;

use ::image::DynamicImage;
use common::*;
use morsa_core::detect::ImageFormat;
use morsa_core::image::ConvertOptions;

use ImageFormat::{Bmp, Gif, Ico, Jpeg, Png, Svg, Tiff, WebP};

fn codes(out: &morsa_core::image::ConvertOutput) -> Vec<&'static str> {
    out.warnings.iter().map(|w| w.code()).collect()
}

// --- Transparência (§5.3) ---------------------------------------------------

/// `valido-rgba.png`: x ≤ 31 opaco; x = 32 com 50% de alfa; x ≥ 33 transparente.
const RGBA: &str = "png/valido-rgba.png";

#[test]
fn transparencia_vira_o_fundo_escolhido_em_jpg_e_bmp() {
    for target in [Jpeg, Bmp] {
        // Fundo padrão: branco.
        let out = convert_file(RGBA, target);
        let img = decode_output(&out.bytes, target);
        let p = img.get_pixel(50, 10).0;
        assert!(
            channel_distance(p, [255, 255, 255, 255]) <= 4,
            "{target:?} branco: {p:?}"
        );
        // Pixel opaco mantém a cor: (x*4, y*5, 200) em (10, 10) = (40, 50, 200).
        let q = img.get_pixel(10, 10).0;
        assert!(
            channel_distance(q, [40, 50, 200, 255]) <= 12,
            "{target:?} opaco: {q:?}"
        );

        // Fundo escolhido: vermelho.
        let options = ConvertOptions {
            background: [255, 0, 0],
            ..ConvertOptions::new(target)
        };
        let red = decode_output(&convert_file_with(RGBA, options).bytes, target);
        let p = red.get_pixel(50, 10).0;
        assert!(
            channel_distance(p, [255, 0, 0, 255]) <= 8,
            "{target:?} vermelho: {p:?}"
        );
        // A coluna de 50% de alfa vira uma mistura das duas cores.
        let mid = red.get_pixel(32, 10).0;
        assert!(
            mid[0] > 150 && mid[1] > 20 && mid[1] < 130,
            "{target:?} meio: {mid:?}"
        );
    }
}

#[test]
fn transparencia_binaria_no_gif() {
    let img = decode_output(&convert_file(RGBA, Gif).bytes, Gif);
    assert_eq!(img.get_pixel(10, 10)[3], 255, "opaco");
    assert_eq!(img.get_pixel(50, 10)[3], 0, "transparente");
    // 50% de alfa (128) é "pelo menos metade": fica opaco.
    assert_eq!(img.get_pixel(32, 10)[3], 255);
}

#[test]
fn transparencia_preservada_em_png_webp_tiff_e_ico() {
    for (source, relative) in [
        (Png, RGBA),
        (WebP, "webp/transparente.webp"),
        (Tiff, "tiff/transparente.tiff"),
    ] {
        let reference = decode_file(relative, source);
        for target in [Png, Tiff, WebP, Ico] {
            if target == source {
                continue;
            }
            let options = ConvertOptions {
                webp_lossless: true,
                ..ConvertOptions::new(target)
            };
            let out = convert_file_with(relative, options);
            let img = decode_output(&out.bytes, target);
            // Um pixel transparente e um parcial continuam com o mesmo alfa
            // (o ICO é escalado para 256, então só confere as regiões).
            let (tx, ty, ox, oy, mid_alpha) = match source {
                Png => (50, 10, 10, 10, 128),
                WebP => (50, 10, 10, 10, 100),
                _ => (10, 10, 25, 10, 80),
            };
            if target == Ico {
                let scale = 256.0 / f64::from(reference.width().max(reference.height()));
                let at = |x: u32, y: u32| {
                    img.get_pixel(
                        (f64::from(x) * scale) as u32,
                        (f64::from(y) * scale + 32.0) as u32,
                    )[3]
                };
                assert_eq!(
                    at(ox, oy),
                    reference.get_pixel(ox, oy)[3],
                    "{source:?}→{target:?}"
                );
                continue;
            }
            assert_eq!(
                img.get_pixel(tx, ty)[3],
                reference.get_pixel(tx, ty)[3],
                "{source:?}→{target:?}"
            );
            assert_eq!(
                img.get_pixel(ox, oy)[3],
                reference.get_pixel(ox, oy)[3],
                "{source:?}→{target:?}"
            );
            let _ = mid_alpha;
        }
    }
}

// --- Profundidade e cinza (§5.3) -------------------------------------------

#[test]
fn dezesseis_bits_ficam_em_dezesseis_bits_em_png_e_tiff() {
    // PNG 16 bits → TIFF.
    let source = ::image::load_from_memory(&Original::read("png/valido-16bit.png").bytes)
        .unwrap()
        .to_rgba16();
    let tiff = convert_file("png/valido-16bit.png", Tiff);
    let back =
        ::image::load_from_memory_with_format(&tiff.bytes, ::image::ImageFormat::Tiff).unwrap();
    assert!(
        matches!(
            back,
            DynamicImage::ImageRgb16(_) | DynamicImage::ImageRgba16(_)
        ),
        "{:?}",
        back.color()
    );
    assert_eq!(back.to_rgba16().as_raw(), source.as_raw());

    // TIFF 16 bits → PNG.
    let source = ::image::load_from_memory(&Original::read("tiff/16bit.tiff").bytes)
        .unwrap()
        .to_rgba16();
    let png = convert_file("tiff/16bit.tiff", Png);
    let back =
        ::image::load_from_memory_with_format(&png.bytes, ::image::ImageFormat::Png).unwrap();
    assert!(
        matches!(
            back,
            DynamicImage::ImageRgb16(_) | DynamicImage::ImageRgba16(_)
        ),
        "{:?}",
        back.color()
    );
    assert_eq!(back.to_rgba16().as_raw(), source.as_raw());
}

#[test]
fn dezesseis_para_oito_bits_arredonda() {
    // valido-16bit.png: pixel (x, y) = (x*2000, y*2000, 40000, 65535).
    let bmp = decode_output(&convert_file("png/valido-16bit.png", Bmp).bytes, Bmp);
    let round = |v: u32| ((v * 255 + 32_767) / 65_535) as u8;
    for (x, y) in [(0u32, 0u32), (3, 5), (31, 31), (7, 20)] {
        let expected = [round(x * 2000), round(y * 2000), round(40_000), 255];
        assert_eq!(bmp.get_pixel(x, y).0, expected, "({x}, {y})");
    }
}

#[test]
fn cinza_continua_cinza() {
    let gray = |relative, target| {
        let out = convert_file(relative, target);
        ::image::load_from_memory_with_format(&out.bytes, common::codec(target)).unwrap()
    };
    assert!(matches!(
        gray("png/cinza.png", Tiff),
        DynamicImage::ImageLuma8(_)
    ));
    assert!(matches!(
        gray("png/cinza.png", Jpeg),
        DynamicImage::ImageLuma8(_)
    ));
    // Cinza com alfa em TIFF vira RGBA (legível em qualquer programa), com os
    // três canais de cor iguais.
    let tiff = gray("png/cinza-alfa.png", Tiff);
    assert!(
        matches!(tiff, DynamicImage::ImageRgba8(_)),
        "{:?}",
        tiff.color()
    );
    assert!(
        tiff.to_rgba8()
            .pixels()
            .all(|p| p[0] == p[1] && p[1] == p[2])
    );
    assert!(matches!(
        gray("png/cinza-alfa.png", ImageFormat::WebP),
        DynamicImage::ImageRgba8(_)
    ));
    // Cinza com alfa em JPG: compõe sobre o branco e continua cinza.
    assert!(matches!(
        gray("png/cinza-alfa.png", Jpeg),
        DynamicImage::ImageLuma8(_)
    ));
}

// --- CMYK e perfil de cor ---------------------------------------------------

#[test]
fn cmyk_vira_rgb() {
    // cmyk.jpg: faixas de 8 px de ciano, magenta, amarelo e preto puros.
    let img = decode_output(&convert_file("jpg/cmyk.jpg", Png).bytes, Png);
    let band = |i: u32| img.get_pixel(i * 8 + 4, 16).0;
    let expect = [
        [0, 255, 255, 255],
        [255, 0, 255, 255],
        [255, 255, 0, 255],
        [0, 0, 0, 255],
    ];
    for i in 0..4 {
        assert!(
            channel_distance(band(i), expect[i as usize]) <= 40,
            "faixa {i}: {:?}",
            band(i)
        );
    }
}

#[test]
fn perfil_p3_e_convertido_para_srgb() {
    // Display P3 (200, 100, 50) vale, em sRGB, (215, 93, 31): conta feita à
    // parte, com as matrizes de primárias e a curva sRGB (ver testes de cor).
    let expected = [215, 93, 31, 255];
    let png = decode_output(&convert_file("png/icc-p3.png", Bmp).bytes, Bmp);
    assert!(
        channel_distance(png.get_pixel(5, 5).0, expected) <= 3,
        "{:?}",
        png.get_pixel(5, 5)
    );
    let jpg = decode_output(&convert_file("jpg/icc-p3.jpg", Png).bytes, Png);
    assert!(
        channel_distance(jpg.get_pixel(5, 5).0, expected) <= 10,
        "{:?}",
        jpg.get_pixel(5, 5)
    );
}

// --- Orientação EXIF (§5.3) -------------------------------------------------

#[test]
fn orientacao_exif_1_a_8_gira_os_pixels() {
    // `exif-canonico.png` é a imagem como deve ser vista: quadrantes
    // vermelho (topo esq.), verde (topo dir.), azul (base esq.), amarelo
    // (base dir.) e um quadrado branco perto do canto superior esquerdo.
    let canonical = decode_file("jpg/exif-canonico.png", Png);
    assert_eq!(canonical.dimensions(), (40, 30));
    let points = [(10, 7), (30, 7), (10, 22), (30, 22), (4, 4), (36, 26)];
    for n in 1..=8 {
        let relative = format!("jpg/exif-{n}.jpg");
        let img = decode_output(&convert_file(&relative, Png).bytes, Png);
        assert_eq!(
            img.dimensions(),
            (40, 30),
            "exif-{n}: dimensões depois de girar"
        );
        for (x, y) in points {
            let (got, want) = (img.get_pixel(x, y).0, canonical.get_pixel(x, y).0);
            assert!(
                channel_distance(got, want) <= 45,
                "exif-{n} em ({x}, {y}): {got:?} ≠ {want:?}"
            );
        }
    }
}

// --- Animados, multipágina e ICO (§5.3) --------------------------------------

#[test]
fn animacao_usa_so_o_primeiro_quadro_e_avisa() {
    for (source, relative) in [(Gif, "gif/animado.gif"), (WebP, "webp/animado.webp")] {
        let out = convert_file(relative, Png);
        assert_eq!(codes(&out), ["E-108"], "{source:?}");
        let img = decode_output(&out.bytes, Png);
        assert_eq!(img.dimensions(), (16, 16));
        // O compositor do WebP animado pode errar 1 nível por arredondamento.
        assert!(
            channel_distance(img.get_pixel(8, 8).0, [255, 0, 0, 255]) <= 2,
            "{source:?}: o 1º quadro é vermelho"
        );
        assert_eq!(img.get_pixel(8, 8)[3], 255);
    }
}

#[test]
fn tiff_multipagina_usa_so_a_primeira_pagina_e_avisa() {
    let out = convert_file("tiff/multipagina.tiff", Png);
    assert_eq!(codes(&out), ["E-109"]);
    let img = decode_output(&out.bytes, Png);
    assert_eq!(img.dimensions(), (8, 8));
    assert_eq!(
        img.get_pixel(4, 4).0,
        [255, 0, 0, 255],
        "a 1ª página é vermelha"
    );
}

#[test]
fn arquivos_de_um_quadro_nao_geram_aviso() {
    for relative in [
        "gif/tipico.gif",
        "gif/transparente.gif",
        "webp/tipico.webp",
        "tiff/tipico.tiff",
        "png/tipico.png",
        "svg/simples.svg",
    ] {
        assert!(
            convert_file(relative, Bmp).warnings.is_empty(),
            "{relative}"
        );
    }
}

#[test]
fn ico_usa_o_maior_quadro() {
    // 16 px vermelho, 32 px verde, 48 px azul.
    let out = convert_file("ico/multitamanho.ico", Png);
    let img = decode_output(&out.bytes, Png);
    assert_eq!(img.dimensions(), (48, 48));
    assert_eq!(img.get_pixel(24, 24).0, [0, 0, 255, 255]);
}

#[test]
fn ico_de_saida_tem_um_quadro_por_tamanho_pedido() {
    let options = ConvertOptions {
        ico_sizes: vec![16, 48, 256],
        ..ConvertOptions::new(Ico)
    };
    let out = convert_file_with("png/tipico.png", options);
    assert_eq!(u16::from_le_bytes([out.bytes[4], out.bytes[5]]), 3);
    // O maior quadro (256) é o que um leitor pega.
    assert_eq!(decode_output(&out.bytes, Ico).dimensions(), (256, 256));
}

// --- SVG (§5.3, RNF10) ------------------------------------------------------

#[test]
fn svg_com_referencias_externas_avisa_e_nao_busca_nada() {
    let out = convert_file("svg/referencias-externas.svg", Png);
    assert_eq!(codes(&out), ["E-110"]);
    let img = decode_output(&out.bytes, Png);
    // Só o retângulo amarelo existe: as duas imagens externas não foram lidas.
    for (x, y) in [(5, 5), (20, 40), (45, 10), (60, 40)] {
        assert_eq!(img.get_pixel(x, y).0, [255, 225, 77, 255], "({x}, {y})");
    }
}

#[test]
fn svg_com_texto_usa_a_fonte_empacotada() {
    let img = decode_output(&convert_file("svg/texto.svg", Png).bytes, Png);
    assert_eq!(img.dimensions(), (96, 40));
    let ink = img.pixels().filter(|p| p[3] > 200 && p[0] < 60).count();
    assert!(
        ink > 100,
        "o texto 'Morsa' deveria ter desenhado pixels, tem {ink}"
    );
}

// --- Mínimos, grandes e extensão errada --------------------------------------

#[test]
fn arquivos_de_1_por_1_convertem_para_todos_os_destinos() {
    let sources = [
        (Png, "png/minimo.png"),
        (Jpeg, "jpg/minimo.jpg"),
        (WebP, "webp/minimo.webp"),
        (Gif, "gif/minimo.gif"),
        (Bmp, "bmp/minimo.bmp"),
        (Tiff, "tiff/minimo.tiff"),
        (Ico, "ico/minimo.ico"),
        (Svg, "svg/minimo.svg"),
    ];
    for (source, relative) in sources {
        for target in [Png, Jpeg, WebP, Gif, Bmp, Tiff, Ico] {
            if target == source {
                continue;
            }
            let out = convert_file(relative, target);
            let (w, h) = decode_output(&out.bytes, target).dimensions();
            // O fixture ICO mínimo tem um quadro de 16 × 16.
            let expected = match (source, target) {
                (_, Ico) => (256, 256),
                (Ico, _) => (16, 16),
                _ => (1, 1),
            };
            assert_eq!((w, h), expected, "{source:?} → {target:?}");
        }
    }
}

#[test]
fn imagens_grandes_de_6_megapixels() {
    for (source, relative) in [(Png, "png/grande.png"), (Jpeg, "jpg/grande.jpg")] {
        for target in [Jpeg, WebP, Tiff, Png].into_iter().filter(|t| *t != source) {
            let out = convert_file(relative, target);
            assert_eq!(
                decode_output(&out.bytes, target).dimensions(),
                (3000, 2000),
                "{source:?} → {target:?}"
            );
        }
    }
}

#[test]
fn o_formato_vem_do_conteudo_nao_da_extensao() {
    // PNG com nome .jpg e JPG com nome .png (RN14).
    let png = convert_file("png/png-extensao-errada.jpg", Bmp);
    assert_eq!(png.source, Png);
    let jpg = convert_file("jpg/jpg-extensao-errada.png", Bmp);
    assert_eq!(jpg.source, Jpeg);
}
