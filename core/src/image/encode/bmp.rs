//! BMP: sem transparência na saída (§5.1): compõe sobre o fundo e grava 24 bits.

use ::image::ExtendedColorType;
use ::image::codecs::bmp::BmpEncoder;

use super::internal;
use crate::error::CoreError;
use crate::image::options::ConvertOptions;
use crate::image::raster::Raster;

pub fn encode(raster: &Raster, options: &ConvertOptions) -> Result<Vec<u8>, CoreError> {
    let rgb = raster.flatten_rgb8(options.background);
    let mut out = Vec::new();
    BmpEncoder::new(&mut out)
        .encode(&rgb, raster.width, raster.height, ExtendedColorType::Rgb8)
        .map_err(internal)?;
    Ok(out)
}
