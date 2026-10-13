//! Arquivos corrompidos e opções inválidas: o núcleo recusa com o código
//! certo, sem panic, sem travar e sem tocar no original (TST02, TST06).

mod common;

use common::*;
use morsa_core::api::{self, ConvertSettings, FileFormat, default_settings, detect_format};
use morsa_core::detect::ImageFormat;
use morsa_core::image::{ConvertOptions, TiffCompression};

use ImageFormat::{Bmp, Gif, Ico, Jpeg, Png, Tiff, WebP};

const ALL_TARGETS: [ImageFormat; 7] = [Png, Jpeg, WebP, Gif, Bmp, Tiff, Ico];

#[test]
fn arquivo_truncado_e_e105_em_todos_os_formatos() {
    for relative in [
        "png/truncado.png",
        "jpg/truncado.jpg",
        "webp/truncado.webp",
        "gif/truncado.gif",
        "bmp/truncado.bmp",
        "tiff/truncado.tiff",
        "ico/truncado.ico",
        "svg/truncado.svg",
    ] {
        for target in ALL_TARGETS {
            let code = error_code(relative, target);
            // O mesmo formato de origem e destino recusa antes de ler (E-101).
            assert!(
                ["E-105", "E-101"].contains(&code.as_str()),
                "{relative} → {target:?}: {code}"
            );
        }
        // Pelo menos um destino diferente do formato de origem leu e falhou.
        assert_eq!(
            error_code(relative, if relative.contains("png/") { Jpeg } else { Png }),
            "E-105",
            "{relative}"
        );
    }
}

#[test]
fn cabecalho_invalido_nunca_converte() {
    // A assinatura confere, o resto é lixo. A maioria dá E-105; GIF lê
    // largura e altura do lixo (gigantes, E-106) e o BMP com dimensões
    // zeradas é recusado pelo leitor como variante não suportada (E-102).
    let cases: [(&str, &[&str]); 8] = [
        ("png/cabecalho-invalido.png", &["E-105"]),
        ("jpg/cabecalho-invalido.jpg", &["E-105"]),
        ("webp/cabecalho-invalido.webp", &["E-105"]),
        ("gif/cabecalho-invalido.gif", &["E-105", "E-106"]),
        ("bmp/cabecalho-invalido.bmp", &["E-102", "E-105"]),
        ("tiff/cabecalho-invalido.tiff", &["E-105"]),
        ("ico/cabecalho-invalido.ico", &["E-105"]),
        ("svg/cabecalho-invalido.svg", &["E-105"]),
    ];
    for (relative, accepted) in cases {
        let target = if relative.contains("png/") { Jpeg } else { Png };
        let code = error_code(relative, target);
        assert!(accepted.contains(&code.as_str()), "{relative}: {code}");
    }
}

#[test]
fn texto_com_extensao_de_imagem_e_e102_e_vazio_e_e103() {
    for target in ALL_TARGETS {
        assert_eq!(
            error_code("png/nao-e-png.png", target),
            "E-102",
            "{target:?}"
        );
        assert_eq!(error_code("png/vazio.png", target), "E-103", "{target:?}");
    }
}

#[test]
fn dimensoes_gigantes_sao_recusadas_antes_de_decodificar() {
    // 40 000 × 40 000 declarados em 65 bytes: E-106 sem alocar pixels.
    let started = std::time::Instant::now();
    for target in ALL_TARGETS.into_iter().filter(|t| *t != Png) {
        assert_eq!(
            error_code("png/dimensoes-gigantes.png", target),
            "E-106",
            "{target:?}"
        );
    }
    for target in ALL_TARGETS {
        assert_eq!(
            error_code("svg/dimensoes-gigantes.svg", target),
            "E-106",
            "{target:?}"
        );
    }
    assert!(
        started.elapsed().as_secs() < 5,
        "a recusa deveria ser imediata"
    );

    // O erro carrega as dimensões para a mensagem (Apêndice A, E-106).
    let original = Original::read("png/dimensoes-gigantes.png");
    let settings = default_settings(FileFormat::Jpeg);
    let error = api::convert_image(original.bytes.clone(), settings).unwrap_err();
    assert_eq!(error.code, "E-106");
    let param = |k: &str| {
        error
            .params
            .iter()
            .find(|p| p.key == k)
            .map(|p| p.value.clone())
    };
    assert_eq!(param("width").as_deref(), Some("40000"));
    assert_eq!(param("height").as_deref(), Some("40000"));
}

#[test]
fn opcoes_invalidas_sao_e104_e_nada_e_gerado() {
    let original = Original::read("png/tipico.png");
    let code = |f: &dyn Fn(&mut ConvertSettings)| {
        let mut s = default_settings(FileFormat::Jpeg);
        f(&mut s);
        api::convert_image(original.bytes.clone(), s)
            .map(|_| ())
            .unwrap_err()
            .code
    };
    assert_eq!(code(&|s| s.quality = 0), "E-104");
    assert_eq!(code(&|s| s.quality = 101), "E-104");
    assert_eq!(code(&|s| s.background = 0x1_00_00_00), "E-104");
    assert_eq!(
        code(&|s| {
            s.target = FileFormat::Ico;
            s.ico_sizes = vec![20];
        }),
        "E-104"
    );
    assert_eq!(
        code(&|s| {
            s.target = FileFormat::Ico;
            s.ico_sizes = vec![];
        }),
        "E-104"
    );
    // O erro diz qual opção e a faixa, para a mensagem destacar o campo.
    let mut s = default_settings(FileFormat::Jpeg);
    s.quality = 0;
    let error = api::convert_image(original.bytes.clone(), s).unwrap_err();
    let get = |k: &str| {
        error
            .params
            .iter()
            .find(|p| p.key == k)
            .map(|p| p.value.clone())
            .unwrap()
    };
    assert_eq!(
        (
            get("option").as_str(),
            get("min").as_str(),
            get("max").as_str()
        ),
        ("quality", "1", "100")
    );
}

#[test]
fn api_converte_e_detecta() {
    let original = Original::read("png/valido-rgba.png");
    let out =
        api::convert_image(original.bytes.clone(), default_settings(FileFormat::Jpeg)).unwrap();
    assert_eq!(
        (out.source, out.target),
        (FileFormat::Png, FileFormat::Jpeg)
    );
    assert_eq!(out.output_extension, "jpg");
    assert_eq!(&out.bytes[..3], &[0xFF, 0xD8, 0xFF]);
    assert!(out.warnings.is_empty());

    // Detecção por conteúdo, só com o cabeçalho (RF02).
    let header = &original.bytes[..original.bytes.len().min(api::detect_header_len() as usize)];
    assert_eq!(detect_format(header.to_vec()).unwrap(), FileFormat::Png);
    assert_eq!(detect_format(vec![]).unwrap_err().code, "E-103");
    assert_eq!(detect_format(b"texto".to_vec()).unwrap_err().code, "E-102");

    // Avisos chegam à interface como códigos.
    let gif = Original::read("gif/animado.gif");
    let out = api::convert_image(gif.bytes.clone(), default_settings(FileFormat::Png)).unwrap();
    assert_eq!(out.warnings, ["E-108"]);
}

#[test]
fn qualidade_muda_o_tamanho_do_jpg_e_do_webp() {
    for target in [Jpeg, WebP] {
        let size = |quality| {
            let options = ConvertOptions {
                quality,
                ..ConvertOptions::new(target)
            };
            convert_file_with("png/grande.png", options).bytes.len()
        };
        let (low, high) = (size(20), size(95));
        assert!(low < high, "{target:?}: q20 {low} B, q95 {high} B");
    }
}

#[test]
fn compressoes_do_tiff_sao_lidas_de_volta_identicas() {
    let reference = decode_file("png/valido-rgba.png", Png);
    let mut sizes = Vec::new();
    for compression in [
        TiffCompression::None,
        TiffCompression::Lzw,
        TiffCompression::Deflate,
    ] {
        let options = ConvertOptions {
            tiff_compression: compression,
            ..ConvertOptions::new(Tiff)
        };
        let out = convert_file_with("png/valido-rgba.png", options);
        assert_eq!(
            decode_output(&out.bytes, Tiff).as_raw(),
            reference.as_raw(),
            "{compression:?}"
        );
        sizes.push(out.bytes.len());
    }
    // Sem compressão é o maior; as duas outras reduzem.
    assert!(sizes[1] < sizes[0] && sizes[2] < sizes[0], "{sizes:?}");
}

#[test]
fn o_original_nunca_e_alterado_nem_quando_a_conversao_falha() {
    // `Original` confere no Drop. Aqui, vários arquivos, bons e ruins.
    for relative in [
        "png/tipico.png",
        "png/truncado.png",
        "jpg/cmyk.jpg",
        "svg/referencias-externas.svg",
        "gif/animado.gif",
    ] {
        let original = Original::read(relative);
        for target in ALL_TARGETS {
            let _ = morsa_core::image::convert(&original.bytes, &ConvertOptions::new(target));
        }
    }
}
