//! PNG: sem perdas, preserva transparência e 16 bits por canal (§5.1).

use ::image::codecs::png::{CompressionType, FilterType, PngEncoder};

use super::internal;
use crate::error::CoreError;
use crate::image::raster::Raster;

pub fn encode(raster: &Raster) -> Result<Vec<u8>, CoreError> {
    let mut out = Vec::new();
    raster
        .to_dynamic()
        .write_with_encoder(PngEncoder::new_with_quality(
            &mut out,
            CompressionType::Default,
            FilterType::Adaptive,
        ))
        .map_err(internal)?;
    Ok(out)
}
