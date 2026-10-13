//! Conversão de uma imagem de um formato para outro.
//!
//! Fluxo: detectar o formato real → conferir a matriz e as opções → decodificar
//! (com limites de segurança, orientação EXIF e perfil de cor) → codificar no
//! formato de destino.

use super::decode;
use super::encode;
use super::options::ConvertOptions;
use crate::detect::{self, ImageFormat};
use crate::error::CoreError;
use crate::warning::Warning;

/// Resultado: os bytes do arquivo novo, os formatos envolvidos e os avisos
/// (animação descartada, página extra, SVG com referências externas).
#[derive(Debug, Clone)]
pub struct ConvertOutput {
    pub bytes: Vec<u8>,
    pub source: ImageFormat,
    pub target: ImageFormat,
    pub warnings: Vec<Warning>,
}

/// Converte `input` conforme `options`.
///
/// `input` é emprestado (`&[u8]`): o Rust garante que esta função não pode
/// modificar os bytes do arquivo original (RN02). A saída é sempre um vetor novo.
pub fn convert(input: &[u8], options: &ConvertOptions) -> Result<ConvertOutput, CoreError> {
    let source = detect::detect(input)?;
    super::check_conversion(source, options.target)?;
    options.validate()?;

    let decoded = decode::decode(input, source, options.svg_size)?;
    let bytes = encode::encode(&decoded.raster, options)?;

    Ok(ConvertOutput {
        bytes,
        source,
        target: options.target,
        warnings: decoded.warnings,
    })
}
