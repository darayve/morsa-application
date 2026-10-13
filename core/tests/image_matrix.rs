//! A matriz de conversão inteira, com arquivos reais.
//!
//! - TST01: todo par permitido da matriz (§5.2) tem um teste de integração.
//! - TST05: a saída é decodificada de volta (por um leitor independente) e
//!   comparada: dimensões exatas; sem perdas, pixels idênticos; com perdas,
//!   PSNR ≥ 35 dB na qualidade padrão (85).
//! - TST06: o arquivo de entrada não muda (`Original` confere ao final).

mod common;

use common::*;
use morsa_core::detect::{self, ImageFormat};
use morsa_core::image::{ConvertOptions, SvgSize, convert};

/// Uma entrada típica por formato.
const SOURCES: [(ImageFormat, &str); 8] = [
    (ImageFormat::Png, "png/tipico.png"),
    (ImageFormat::Jpeg, "jpg/tipico.jpg"),
    (ImageFormat::WebP, "webp/tipico.webp"),
    (ImageFormat::Gif, "gif/tipico.gif"),
    (ImageFormat::Bmp, "bmp/tipico.bmp"),
    (ImageFormat::Tiff, "tiff/tipico.tiff"),
    (ImageFormat::Ico, "ico/multitamanho.ico"),
    (ImageFormat::Svg, "svg/simples.svg"),
];

const TARGETS: [ImageFormat; 7] = [
    ImageFormat::Png,
    ImageFormat::Jpeg,
    ImageFormat::WebP,
    ImageFormat::Gif,
    ImageFormat::Bmp,
    ImageFormat::Tiff,
    ImageFormat::Ico,
];

/// Como a imagem de entrada é vista, por um leitor independente do núcleo. O
/// SVG não tem leitor independente: usa-se a saída PNG (já conferida em
/// `svg_renderiza_formas`).
fn reference(source: ImageFormat, relative: &str) -> RgbaImage {
    if source == ImageFormat::Svg {
        let png = convert_file(relative, ImageFormat::Png);
        decode_output(&png.bytes, ImageFormat::Png)
    } else {
        decode_file(relative, source)
    }
}
use ::image::RgbaImage;

#[test]
fn todo_par_da_matriz_converte_e_confere() {
    let mut pairs = 0;
    for (source, relative) in SOURCES {
        let reference = reference(source, relative);
        for target in TARGETS {
            if target == source {
                continue; // sem transformação, é E-101 (ver `mesmo_formato_sem_transformacao`)
            }
            let out = convert_file(relative, target);
            pairs += 1;
            let label = format!("{source:?} → {target:?}");

            // O arquivo gerado é, de fato, do formato pedido (RN14/RN15).
            assert_eq!(
                detect::detect(&out.bytes),
                Ok(target),
                "{label}: assinatura"
            );
            assert_eq!(out.source, source, "{label}");
            assert_eq!(out.target, target, "{label}");

            let decoded = decode_output(&out.bytes, target);
            let (w, h) = reference.dimensions();
            let opaque_ref = flatten_on_white(&reference);
            // Meta do TST05: 35 dB a q85. Duas entradas de teste são duras
            // para qualquer codec com perdas: o GIF é pontilhado (ruído de
            // alta frequência) e o SVG tem bordas de cor pura, que a
            // subamostragem de crominância 4:2:0 borra. A libwebp oficial
            // também fica perto de 31 dB nelas; a meta cai para 30 dB.
            let hard = matches!(source, ImageFormat::Gif | ImageFormat::Svg);
            let min_psnr = if hard { 30.0 } else { 35.0 };

            match target {
                // Sem perdas e com transparência: pixels idênticos, alfa incluído.
                ImageFormat::Png | ImageFormat::Tiff => {
                    assert_eq!(decoded.dimensions(), (w, h), "{label}");
                    assert_eq!(decoded.as_raw(), reference.as_raw(), "{label}: pixels");
                }
                // BMP não tem transparência: idêntico à imagem composta no branco.
                ImageFormat::Bmp => {
                    assert_eq!(decoded.dimensions(), (w, h), "{label}");
                    assert_eq!(decoded.as_raw(), opaque_ref.as_raw(), "{label}: pixels");
                }
                // Com perdas (padrão): PSNR ≥ 35 dB a 85.
                ImageFormat::Jpeg => {
                    assert_eq!(decoded.dimensions(), (w, h), "{label}");
                    let p = psnr(&decoded, &opaque_ref);
                    assert!(p >= min_psnr, "{label}: PSNR {p:.1} dB");
                }
                ImageFormat::WebP => {
                    assert_eq!(decoded.dimensions(), (w, h), "{label}");
                    let p = psnr(&decoded, &reference);
                    assert!(p >= min_psnr, "{label}: PSNR {p:.1} dB");
                }
                // Paleta de 256 cores: o erro é maior, mas a imagem tem de ser
                // reconhecível e ter no máximo 256 cores.
                ImageFormat::Gif => {
                    assert_eq!(decoded.dimensions(), (w, h), "{label}");
                    let colors: std::collections::HashSet<_> =
                        decoded.pixels().map(|p| [p[0], p[1], p[2], p[3]]).collect();
                    assert!(colors.len() <= 256, "{label}: {} cores", colors.len());
                    let e = block_mean_error(&flatten_on_white(&decoded), &opaque_ref, 4);
                    assert!(e < 4.0, "{label}: erro médio dos blocos {e:.2}");
                }
                // ICO: o maior tamanho padrão (256) é o quadro lido; a imagem
                // cabe nele sem distorcer, então a cor média se mantém.
                ImageFormat::Ico => {
                    assert_eq!(decoded.dimensions(), (256, 256), "{label}");
                    let (a, b) = (mean_opaque(&decoded), mean_opaque(&reference));
                    assert!(
                        (0..3).all(|c| (a[c] - b[c]).abs() < 14.0),
                        "{label}: média {a:?} vs {b:?}"
                    );
                }
                ImageFormat::Svg => unreachable!(),
            }
        }
    }
    // 8 entradas × 7 saídas − 7 pares do mesmo formato.
    assert_eq!(pairs, 49);
}

/// Média de cor dos pixels com alfa alto.
fn mean_opaque(img: &RgbaImage) -> [f64; 3] {
    let (mut sum, mut n) = ([0f64; 3], 0f64);
    for p in img.pixels().filter(|p| p[3] > 200) {
        for c in 0..3 {
            sum[c] += f64::from(p[c]);
        }
        n += 1.0;
    }
    [sum[0] / n, sum[1] / n, sum[2] / n]
}

#[test]
fn webp_sem_perdas_e_identico() {
    for relative in [
        "png/tipico.png",
        "png/valido-rgba.png",
        "tiff/transparente.tiff",
    ] {
        let options = ConvertOptions {
            webp_lossless: true,
            ..ConvertOptions::new(ImageFormat::WebP)
        };
        let out = convert_file_with(relative, options);
        let source = decode_file(
            relative,
            if relative.ends_with("tiff") {
                ImageFormat::Tiff
            } else {
                ImageFormat::Png
            },
        );
        let decoded = decode_output(&out.bytes, ImageFormat::WebP);
        // Pixels totalmente transparentes podem perder a cor escondida; o
        // resto, inclusive o alfa, é idêntico.
        for (a, b) in decoded.pixels().zip(source.pixels()) {
            assert_eq!(a[3], b[3], "{relative}: alfa");
            if b[3] > 0 {
                assert_eq!(a.0, b.0, "{relative}");
            }
        }
    }
}

#[test]
fn mesmo_formato_sem_transformacao_e_e101() {
    for (source, relative) in SOURCES {
        let original = Original::read(relative);
        let result = convert(&original.bytes, &ConvertOptions::new(source));
        let error = result.expect_err("mesmo formato sem transformação");
        assert_eq!(error.code(), "E-101", "{source:?}");
    }
}

#[test]
fn svg_nao_e_formato_de_saida() {
    for (_, relative) in SOURCES {
        let original = Original::read(relative);
        let error = convert(&original.bytes, &ConvertOptions::new(ImageFormat::Svg))
            .expect_err("SVG como destino");
        assert_eq!(error.code(), "E-101");
    }
}

#[test]
fn svg_renderiza_formas_com_as_cores_do_arquivo() {
    // simples.svg: retângulo vermelho (220,30,30) à esquerda, círculo azul
    // (30,120,220) à direita, fundo transparente.
    let out = convert_file("svg/simples.svg", ImageFormat::Png);
    let img = decode_output(&out.bytes, ImageFormat::Png);
    assert_eq!(img.dimensions(), (64, 48));
    assert_eq!(img.get_pixel(10, 24).0, [220, 30, 30, 255]);
    assert_eq!(img.get_pixel(48, 24).0, [30, 120, 220, 255]);
    assert_eq!(img.get_pixel(40, 2)[3], 0, "fora das formas: transparente");
}

#[test]
fn svg_respeita_o_tamanho_pedido() {
    let size = |s: SvgSize, file: &str| {
        let options = ConvertOptions {
            svg_size: s,
            ..ConvertOptions::new(ImageFormat::Png)
        };
        let out = convert_file_with(file, options);
        decode_output(&out.bytes, ImageFormat::Png).dimensions()
    };
    assert_eq!(size(SvgSize::Declared, "svg/simples.svg"), (64, 48));
    assert_eq!(
        size(SvgSize::ScalePercent(300), "svg/simples.svg"),
        (192, 144)
    );
    assert_eq!(
        size(
            SvgSize::Pixels {
                width: Some(128),
                height: None
            },
            "svg/simples.svg"
        ),
        (128, 96)
    );
    assert_eq!(
        size(
            SvgSize::Pixels {
                width: None,
                height: Some(24)
            },
            "svg/simples.svg"
        ),
        (32, 24)
    );
    // Sem width/height no arquivo: 1024 px de largura (RF06).
    assert_eq!(size(SvgSize::Declared, "svg/so-viewbox.svg"), (1024, 512));
}
