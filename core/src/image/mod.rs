//! Imagens: decodificar, transformar e codificar.
//!
//! Neste módulo, `::image` (com `::` na frente) é o crate externo `image`;
//! sem os `::`, `image` seria este próprio módulo.

mod convert;

pub use convert::{ConvertOptions, ConvertOutput, DEFAULT_JPEG_QUALITY, convert};

use crate::detect::ImageFormat;
use crate::error::CoreError;

/// Matriz de conversão (requisitos §5.2), a única fonte de verdade (RN01).
///
/// Converter para o mesmo formato só é permitido quando há alguma
/// transformação; como o Marco 1 ainda não tem transformações, o mesmo
/// formato é sempre recusado com `E-101`.
pub fn check_conversion(from: ImageFormat, to: ImageFormat) -> Result<(), CoreError> {
    if from == to {
        return Err(CoreError::UnsupportedConversion {
            from: from.label().into(),
            to: to.label().into(),
        });
    }
    Ok(())
}
