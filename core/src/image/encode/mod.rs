//! Codificadores: [`Raster`] → bytes de cada formato de saída (§5.1).
//!
//! Cada formato tem seu arquivo. Todos recebem a imagem já normalizada e as
//! opções, e devolvem os bytes de um arquivo novo.

mod bmp;
mod gif;
mod ico;
mod jpeg;
mod png;
mod tiff;
mod webp;

use std::fmt::Display;

use super::options::ConvertOptions;
use super::raster::Raster;
use crate::detect::ImageFormat;
use crate::error::CoreError;

pub fn encode(raster: &Raster, options: &ConvertOptions) -> Result<Vec<u8>, CoreError> {
    match options.target {
        ImageFormat::Png => png::encode(raster),
        ImageFormat::Jpeg => jpeg::encode(raster, options),
        ImageFormat::WebP => webp::encode(raster, options),
        ImageFormat::Gif => gif::encode(raster),
        ImageFormat::Bmp => bmp::encode(raster, options),
        ImageFormat::Tiff => tiff::encode(raster, options),
        ImageFormat::Ico => ico::encode(raster, options),
        // SVG não é formato de saída; `convert` já recusa antes de chegar aqui.
        ImageFormat::Svg => Err(CoreError::UnsupportedConversion {
            from: "?".into(),
            to: "SVG".into(),
        }),
    }
}

/// Erro de um codificador, que não é culpa do arquivo de entrada.
fn internal(e: impl Display) -> CoreError {
    CoreError::Internal {
        detail: e.to_string(),
    }
}

/// O formato de saída não comporta imagens tão grandes (WebP: 16 383 px por
/// lado, GIF e JPG: 65 535).
fn check_side(raster: &Raster, max: u32) -> Result<(), CoreError> {
    if raster.width > max || raster.height > max {
        return Err(CoreError::ImageTooLarge {
            width: raster.width,
            height: raster.height,
        });
    }
    Ok(())
}
