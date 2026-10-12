//! Testes de integração da conversão PNG → JPG com arquivos reais do corpus.
//!
//! Cada teste lê um arquivo de `corpus/`, chama a API do núcleo e confere o
//! resultado. `read_corpus` também guarda os bytes do original para provar,
//! no fim, que o arquivo em disco não mudou (RN02 / TST06).

use std::fs;
use std::path::PathBuf;

use morsa_core::api::{FileFormat, convert_image, detect_format};

fn corpus_path(relative: &str) -> PathBuf {
    // CARGO_MANIFEST_DIR é a pasta core/; o corpus fica um nível acima.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../corpus")
        .join(relative)
}

/// Lê o arquivo e devolve uma "testemunha" que confere, ao ser descartada,
/// que o arquivo em disco continua idêntico.
struct Original {
    path: PathBuf,
    bytes: Vec<u8>,
}

impl Original {
    fn read(relative: &str) -> Self {
        let path = corpus_path(relative);
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        Original { path, bytes }
    }
}

impl Drop for Original {
    fn drop(&mut self) {
        let after = fs::read(&self.path).unwrap();
        assert_eq!(
            after,
            self.bytes,
            "o original foi alterado: {}",
            self.path.display()
        );
    }
}

fn error_code(relative: &str, target: FileFormat, quality: u8) -> String {
    let original = Original::read(relative);
    match convert_image(original.bytes.clone(), target, quality) {
        Ok(_) => panic!("{relative} deveria falhar"),
        Err(e) => e.code,
    }
}

#[test]
fn png_valido_vira_jpg_com_mesmas_dimensoes() {
    let original = Original::read("image/png/valido-rgba.png");
    let out = convert_image(original.bytes.clone(), FileFormat::Jpeg, 85).unwrap();

    assert_eq!(out.source, FileFormat::Png);
    assert_eq!(out.target, FileFormat::Jpeg);
    assert_eq!(out.extension, "jpg");
    assert_eq!(
        &out.bytes[..3],
        &[0xFF, 0xD8, 0xFF],
        "saída não começa como JPG"
    );

    let decoded = image::load_from_memory_with_format(&out.bytes, image::ImageFormat::Jpeg)
        .unwrap()
        .to_rgb8();
    assert_eq!((decoded.width(), decoded.height()), (64, 48));

    // Metade direita era transparente: precisa sair branca (fundo padrão, §5.3).
    // JPG tem perdas, então aceitamos uma pequena margem.
    let px = decoded.get_pixel(50, 20);
    assert!(
        px.0.iter().all(|&c| c >= 245),
        "esperado branco, veio {px:?}"
    );

    // A metade esquerda era opaca e azulada: não pode ter virado branco.
    let px = decoded.get_pixel(5, 20);
    assert!(px.0[0] < 100, "pixel opaco perdeu a cor: {px:?}");
}

#[test]
fn png_16_bits_converte() {
    let original = Original::read("image/png/valido-16bit.png");
    let out = convert_image(original.bytes.clone(), FileFormat::Jpeg, 85).unwrap();
    let decoded = image::load_from_memory(&out.bytes).unwrap();
    assert_eq!((decoded.width(), decoded.height()), (32, 32));
}

#[test]
fn extensao_errada_usa_o_formato_real() {
    // RN14: o arquivo se chama .jpg, mas é PNG.
    let original = Original::read("image/png/png-extensao-errada.jpg");
    assert_eq!(
        detect_format(original.bytes.clone()).ok(),
        Some(FileFormat::Png)
    );
    let out = convert_image(original.bytes.clone(), FileFormat::Jpeg, 85).unwrap();
    assert_eq!(out.source, FileFormat::Png);
}

#[test]
fn qualidade_muda_o_resultado() {
    let original = Original::read("image/png/valido-rgba.png");
    let low = convert_image(original.bytes.clone(), FileFormat::Jpeg, 10).unwrap();
    let high = convert_image(original.bytes.clone(), FileFormat::Jpeg, 100).unwrap();
    assert!(low.bytes.len() < high.bytes.len());
}

#[test]
fn truncado_da_e105() {
    assert_eq!(
        error_code("image/png/truncado.png", FileFormat::Jpeg, 85),
        "E-105"
    );
}

#[test]
fn cabecalho_invalido_da_e105() {
    assert_eq!(
        error_code("image/png/cabecalho-invalido.png", FileFormat::Jpeg, 85),
        "E-105"
    );
}

#[test]
fn texto_com_extensao_png_da_e102() {
    assert_eq!(
        error_code("image/png/nao-e-png.png", FileFormat::Jpeg, 85),
        "E-102"
    );
}

#[test]
fn vazio_da_e103() {
    assert_eq!(
        error_code("image/png/vazio.png", FileFormat::Jpeg, 85),
        "E-103"
    );
}

#[test]
fn dimensoes_gigantes_dao_e106_sem_decodificar() {
    let original = Original::read("image/png/dimensoes-gigantes.png");
    let error =
        convert_image(original.bytes.clone(), FileFormat::Jpeg, 85).expect_err("deveria falhar");
    assert_eq!(error.code, "E-106");
    let width = error.params.iter().find(|p| p.key == "width").unwrap();
    assert_eq!(width.value, "40000");
}

#[test]
fn qualidade_fora_da_faixa_da_e104() {
    assert_eq!(
        error_code("image/png/valido-rgba.png", FileFormat::Jpeg, 0),
        "E-104"
    );
    assert_eq!(
        error_code("image/png/valido-rgba.png", FileFormat::Jpeg, 101),
        "E-104"
    );
}

#[test]
fn mesmo_formato_sem_transformacao_da_e101() {
    // RN01: o núcleo recusa mesmo que a interface ofereça por engano.
    assert_eq!(
        error_code("image/jpg/valido.jpg", FileFormat::Jpeg, 85),
        "E-101"
    );
    assert_eq!(
        error_code("image/png/valido-rgba.png", FileFormat::Png, 85),
        "E-101"
    );
}
