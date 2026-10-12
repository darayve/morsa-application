//! Funções e tipos expostos ao Flutter pelo flutter_rust_bridge.
//!
//! Este é o único arquivo do núcleo que conhece a ponte. Ele traduz os tipos
//! internos para tipos simples (enums, structs, `Vec<u8>`) que o gerador
//! transforma em Dart, e captura qualquer panic para que uma falha do núcleo
//! vire um erro do item (`E-199`) em vez de derrubar o app (RNF12).
//!
//! No Dart, toda função daqui é assíncrona: o FRB a executa fora da thread
//! da interface (thread nativa no desktop, Web Worker na web).

use std::panic::{self, AssertUnwindSafe};

use flutter_rust_bridge::frb;

use crate::detect::{self, ImageFormat};
use crate::error::CoreError;
use crate::image::{self, ConvertOptions};

/// Formato de arquivo, como visto pela interface.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Png,
    Jpeg,
}

/// Imagem convertida.
#[derive(Debug)]
pub struct ConvertedImage {
    pub bytes: Vec<u8>,
    pub source: FileFormat,
    pub target: FileFormat,
    /// Extensão da saída sem o ponto (`jpg`), conforme RN15.
    pub output_extension: String,
}

/// Par nome → valor usado para preencher a mensagem traduzida do erro.
#[derive(Debug)]
pub struct ErrorParam {
    pub key: String,
    pub value: String,
}

/// Erro devolvido ao Dart. A interface usa `code` (`E-105`…) para a mensagem
/// em linguagem simples; `detail` é técnico e vai só para o relatório.
#[derive(Debug)]
pub struct MorsaError {
    pub code: String,
    pub params: Vec<ErrorParam>,
    pub detail: String,
}

/// Chamada uma vez pelo FRB ao iniciar o app.
#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// Qualidade padrão do JPG, para a interface não duplicar o valor.
#[frb(sync)]
pub fn default_jpeg_quality() -> u8 {
    image::DEFAULT_JPEG_QUALITY
}

/// Quantos bytes do início do arquivo `detect_format` precisa.
#[frb(sync)]
pub fn detect_header_len() -> u32 {
    detect::HEADER_LEN as u32
}

/// Identifica o formato real pelo início do arquivo (RF02).
pub fn detect_format(header: Vec<u8>) -> Result<FileFormat, MorsaError> {
    guard(|| detect::detect(&header).map(FileFormat::from))
}

/// Converte uma imagem. Recebe os bytes do original e devolve um arquivo novo.
pub fn convert_image(
    input: Vec<u8>,
    target: FileFormat,
    quality: u8,
) -> Result<ConvertedImage, MorsaError> {
    guard(|| {
        let options = ConvertOptions {
            target: target.into(),
            quality,
        };
        let out = image::convert(&input, options)?;
        Ok(ConvertedImage {
            output_extension: out.target.extension().to_string(),
            bytes: out.bytes,
            source: out.source.into(),
            target: out.target.into(),
        })
    })
}

/// Executa `work` capturando panics. Um panic vira `E-199`.
///
/// `AssertUnwindSafe` diz ao compilador que não há estado compartilhado que
/// possa ficar inconsistente depois de um panic: cada chamada só usa os
/// próprios dados.
///
/// Na web, o WASM é compilado com `panic = abort` e não dá para capturar;
/// lá o FRB devolve a falha como exceção e o Dart a trata como `E-199`.
fn guard<T>(work: impl FnOnce() -> Result<T, CoreError>) -> Result<T, MorsaError> {
    match panic::catch_unwind(AssertUnwindSafe(work)) {
        Ok(result) => result.map_err(MorsaError::from),
        Err(payload) => {
            let detail = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "panic".to_string());
            Err(MorsaError::from(CoreError::Internal { detail }))
        }
    }
}

impl From<CoreError> for MorsaError {
    fn from(error: CoreError) -> Self {
        MorsaError {
            code: error.code().to_string(),
            params: error
                .params()
                .into_iter()
                .map(|(key, value)| ErrorParam { key, value })
                .collect(),
            detail: error.to_string(),
        }
    }
}

impl From<ImageFormat> for FileFormat {
    fn from(format: ImageFormat) -> Self {
        match format {
            ImageFormat::Png => FileFormat::Png,
            ImageFormat::Jpeg => FileFormat::Jpeg,
        }
    }
}

impl From<FileFormat> for ImageFormat {
    fn from(format: FileFormat) -> Self {
        match format {
            FileFormat::Png => ImageFormat::Png,
            FileFormat::Jpeg => ImageFormat::Jpeg,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panic_vira_e199() {
        let result: Result<(), MorsaError> = guard(|| panic!("boom"));
        let error = result.expect_err("deveria falhar");
        assert_eq!(error.code, "E-199");
        assert!(error.detail.contains("boom"));
    }
}
