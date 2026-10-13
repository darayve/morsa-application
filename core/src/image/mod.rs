//! Imagens: decodificar, transformar e codificar.
//!
//! Neste módulo, `::image` (com `::` na frente) é o crate externo `image`;
//! sem os `::`, `image` seria este próprio módulo.

mod color;
mod convert;
mod decode;
mod encode;
mod options;
mod raster;
mod resize;
mod svg;

pub use convert::{ConvertOutput, convert};
pub use options::{
    ConvertOptions, DEFAULT_ICO_SIZES, DEFAULT_QUALITY, ICO_SIZES, MAX_SVG_SIDE, SvgSize,
    TiffCompression,
};

use crate::detect::ImageFormat;
use crate::error::CoreError;

/// Matriz de conversão (requisitos §5.2), a única fonte de verdade (RN01).
///
/// Toda imagem de entrada vai para qualquer formato de imagem de saída, exceto
/// SVG, que só existe como entrada. Converter para o mesmo formato só é
/// permitido quando há alguma transformação; como ainda não há transformações
/// (entram no Marco 3b), o mesmo formato é sempre recusado com `E-101`.
pub fn check_conversion(from: ImageFormat, to: ImageFormat) -> Result<(), CoreError> {
    if from == to || !to.can_be_output() {
        return Err(CoreError::UnsupportedConversion {
            from: from.label().into(),
            to: to.label().into(),
        });
    }
    Ok(())
}
