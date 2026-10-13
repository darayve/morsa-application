//! TIFF: sem perdas, preserva transparência e 16 bits por canal; compressão
//! nenhuma, LZW ou Deflate (RF06).

use std::io::Cursor;

use ::tiff::encoder::Compression;
use ::tiff::encoder::{DeflateLevel, TiffEncoder, colortype};
use ::tiff::tags::ExtraSamples;

use super::internal;
use crate::error::CoreError;
use crate::image::options::{ConvertOptions, TiffCompression};
use crate::image::raster::{Raster, Samples};

pub fn encode(raster: &Raster, options: &ConvertOptions) -> Result<Vec<u8>, CoreError> {
    let compression = match options.tiff_compression {
        TiffCompression::None => Compression::Uncompressed,
        TiffCompression::Lzw => Compression::Lzw,
        TiffCompression::Deflate => Compression::Deflate(DeflateLevel::Balanced),
    };
    let mut buffer = Cursor::new(Vec::new());
    let mut encoder = TiffEncoder::new(&mut buffer)
        .map_err(internal)?
        .with_compression(compression);

    let (w, h) = (raster.width, raster.height);
    let alpha = raster.has_alpha;
    // Cinza com alfa vira RGBA: TIFF "cinza + canal extra" é válido, mas muitos
    // leitores (inclusive o crate `image`) não o interpretam.
    let gray = raster.gray && !alpha;

    // `colortype` diz quantos canais de cor há; o alfa entra como canal
    // extra "não associado" (cor reta, sem pré-multiplicar), que é o que
    // outros programas esperam.
    macro_rules! write {
        ($color:ty, $data:expr) => {{
            let mut image = encoder.new_image::<$color>(w, h).map_err(internal)?;
            if alpha {
                image
                    .extra_samples(&[ExtraSamples::UnassociatedAlpha])
                    .map_err(internal)?;
            }
            image.write_data(&$data).map_err(internal)?;
        }};
    }
    match &raster.samples {
        Samples::U8(p) => {
            let data = pack(p, gray, alpha);
            if gray {
                write!(colortype::Gray8, data)
            } else {
                write!(colortype::RGB8, data)
            }
        }
        Samples::U16(p) => {
            let data = pack(p, gray, alpha);
            if gray {
                write!(colortype::Gray16, data)
            } else {
                write!(colortype::RGB16, data)
            }
        }
    }
    Ok(buffer.into_inner())
}

/// RGBA → só os canais que a saída usa: cinza ou RGB, com ou sem alfa.
fn pack<T: Copy>(rgba: &[T], gray: bool, alpha: bool) -> Vec<T> {
    rgba.chunks_exact(4)
        .flat_map(|p| match (gray, alpha) {
            (true, false) => vec![p[0]],
            (true, true) => vec![p[0], p[3]],
            (false, false) => vec![p[0], p[1], p[2]],
            (false, true) => vec![p[0], p[1], p[2], p[3]],
        })
        .collect()
}
