//! JPG: com perdas, sem transparência (compõe sobre o fundo), qualidade 1–100.

use ::image::codecs::jpeg::JpegEncoder;
use ::image::{ExtendedColorType, ImageEncoder};

use super::{check_side, internal};
use crate::error::CoreError;
use crate::image::options::ConvertOptions;
use crate::image::raster::Raster;

pub fn encode(raster: &Raster, options: &ConvertOptions) -> Result<Vec<u8>, CoreError> {
    check_side(raster, 65_535)?;
    let bg = options.background;
    // Imagem cinza com fundo cinza vira JPG de 1 canal, bem menor.
    let gray_bg = bg[0] == bg[1] && bg[1] == bg[2];
    let (data, color) = if raster.gray && (!raster.has_alpha || gray_bg) {
        (raster.flatten_gray8(bg[0]), ExtendedColorType::L8)
    } else {
        (raster.flatten_rgb8(bg), ExtendedColorType::Rgb8)
    };

    let mut out = Vec::new();
    JpegEncoder::new_with_quality(&mut out, options.quality)
        .write_image(&data, raster.width, raster.height, color)
        .map_err(internal)?;
    Ok(out)
}
