//! WebP: com perdas (VP8) ou sem perdas (VP8L), com transparência (§5.1).
//!
//! - **Sem perdas**: codificador do crate `image`, em Rust puro.
//! - **Com perdas**: crate `oxideav-webp`, também em Rust puro. A `libwebp`
//!   oficial é código C e não compila para WebAssembly sem uma toolchain
//!   extra (decisão DC17); com Rust puro, desktop e web geram arquivos iguais.
//!   O codificador da crate só cuida da imagem de cor. A transparência é
//!   gravada aqui, num chunk `ALPH` sem compressão (RFC 9649 §2.7.1.2),
//!   dentro de um contêiner `VP8X`, porque o caminho de alfa da crate é lento
//!   demais (segundos por megapixel).

use ::image::codecs::webp::WebPEncoder;
use ::image::{ExtendedColorType, ImageEncoder};

use super::{check_side, internal};
use crate::error::CoreError;
use crate::image::options::ConvertOptions;
use crate::image::raster::Raster;

/// O WebP guarda largura e altura em 14 bits.
const MAX_SIDE: u32 = 16_383;

pub fn encode(raster: &Raster, options: &ConvertOptions) -> Result<Vec<u8>, CoreError> {
    check_side(raster, MAX_SIDE)?;
    if options.webp_lossless {
        lossless(raster)
    } else {
        lossy(raster, options.quality)
    }
}

fn lossless(raster: &Raster) -> Result<Vec<u8>, CoreError> {
    let rgba = raster.rgba8();
    let (data, color) = if raster.has_alpha {
        (rgba.into_owned(), ExtendedColorType::Rgba8)
    } else {
        (strip_alpha(&rgba), ExtendedColorType::Rgb8)
    };
    let mut out = Vec::new();
    WebPEncoder::new_lossless(&mut out)
        .write_image(&data, raster.width, raster.height, color)
        .map_err(internal)?;
    Ok(out)
}

fn lossy(raster: &Raster, quality: u8) -> Result<Vec<u8>, CoreError> {
    let rgba = raster.rgba8();
    let rgb = strip_alpha(&rgba);
    let options = oxideav_webp::EncodeOptions::default().with_quality(f32::from(quality));
    let color =
        oxideav_webp::encode_rgb8(raster.width, raster.height, &rgb, &options).map_err(internal)?;
    if !raster.has_alpha {
        return Ok(color);
    }
    let alpha: Vec<u8> = rgba.chunks_exact(4).map(|p| p[3]).collect();
    add_alpha(&color, &alpha, raster.width, raster.height)
}

fn strip_alpha(rgba: &[u8]) -> Vec<u8> {
    rgba.chunks_exact(4)
        .flat_map(|p| [p[0], p[1], p[2]])
        .collect()
}

/// Transforma um WebP com perdas simples (`RIFF` + `VP8 `) num WebP estendido
/// (`RIFF` + `VP8X` + `ALPH` + `VP8 `) com o canal alfa.
fn add_alpha(simple: &[u8], alpha: &[u8], width: u32, height: u32) -> Result<Vec<u8>, CoreError> {
    let vp8 = find_chunk(simple, b"VP8 ").ok_or_else(|| internal("WebP sem chunk VP8"))?;

    let mut body = Vec::with_capacity(vp8.len() + alpha.len() + 64);
    body.extend_from_slice(b"WEBP");

    // VP8X: flags (bit 0x10 = há alfa), 3 bytes reservados, tela (largura − 1
    // e altura − 1, 24 bits cada, little endian).
    let mut vp8x = vec![0x10, 0, 0, 0];
    vp8x.extend_from_slice(&(width - 1).to_le_bytes()[..3]);
    vp8x.extend_from_slice(&(height - 1).to_le_bytes()[..3]);
    push_chunk(&mut body, b"VP8X", &vp8x);

    // ALPH: 1 byte de cabeçalho (0 = sem compressão, sem filtro, sem
    // pré-processamento) seguido de um byte por pixel.
    let mut alph = Vec::with_capacity(alpha.len() + 1);
    alph.push(0);
    alph.extend_from_slice(alpha);
    push_chunk(&mut body, b"ALPH", &alph);

    push_chunk(&mut body, b"VP8 ", vp8);

    let mut out = Vec::with_capacity(body.len() + 8);
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    Ok(out)
}

/// Escreve um chunk RIFF: nome, tamanho (little endian), dados e, se o
/// tamanho for ímpar, 1 byte de preenchimento.
fn push_chunk(out: &mut Vec<u8>, name: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(name);
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    if data.len() % 2 == 1 {
        out.push(0);
    }
}

/// Dados do primeiro chunk com o nome dado, num arquivo RIFF/WEBP.
fn find_chunk<'a>(file: &'a [u8], name: &[u8; 4]) -> Option<&'a [u8]> {
    if file.len() < 12 || &file[0..4] != b"RIFF" || &file[8..12] != b"WEBP" {
        return None;
    }
    let mut pos = 12;
    while pos + 8 <= file.len() {
        let size = u32::from_le_bytes(file[pos + 4..pos + 8].try_into().ok()?) as usize;
        let data = file.get(pos + 8..pos + 8 + size)?;
        if &file[pos..pos + 4] == name {
            return Some(data);
        }
        pos += 8 + size + (size % 2);
    }
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::ImageFormat;
    use crate::image::raster::Samples;

    fn decode(bytes: &[u8]) -> ::image::RgbaImage {
        ::image::load_from_memory_with_format(bytes, ::image::ImageFormat::WebP)
            .unwrap()
            .to_rgba8()
    }

    fn gradient(w: u32, h: u32, alpha: impl Fn(u32, u32) -> u8) -> Raster {
        let mut px = Vec::new();
        for y in 0..h {
            for x in 0..w {
                px.extend_from_slice(&[(x * 255 / w) as u8, (y * 255 / h) as u8, 90, alpha(x, y)]);
            }
        }
        Raster::from_samples(w, h, Samples::U8(px))
    }

    #[test]
    fn sem_perdas_e_identico_inclusive_o_alfa() {
        let r = gradient(40, 30, |x, _| if x < 20 { 255 } else { 100 });
        let opts = ConvertOptions {
            webp_lossless: true,
            ..ConvertOptions::new(ImageFormat::WebP)
        };
        let out = decode(&encode(&r, &opts).unwrap());
        let Samples::U8(px) = &r.samples else {
            panic!()
        };
        assert_eq!(out.as_raw(), px);
    }

    #[test]
    fn com_perdas_mantem_dimensoes_e_alfa() {
        let r = gradient(64, 48, |x, _| if x < 32 { 255 } else { 128 });
        let out = decode(&encode(&r, &ConvertOptions::new(ImageFormat::WebP)).unwrap());
        assert_eq!((out.width(), out.height()), (64, 48));
        assert_eq!(out.get_pixel(5, 5)[3], 255);
        assert_eq!(out.get_pixel(50, 5)[3], 128);
    }

    #[test]
    fn com_perdas_sem_transparencia_nao_tem_chunk_alph() {
        let r = gradient(32, 32, |_, _| 255);
        let bytes = encode(&r, &ConvertOptions::new(ImageFormat::WebP)).unwrap();
        assert!(find_chunk(&bytes, b"ALPH").is_none());
        assert!(find_chunk(&bytes, b"VP8 ").is_some());
    }

    #[test]
    fn lado_acima_de_16383_e_recusado() {
        let r = Raster::from_samples(16_384, 1, Samples::U8(vec![0; 16_384 * 4]));
        assert!(matches!(
            encode(&r, &ConvertOptions::new(ImageFormat::WebP)),
            Err(CoreError::ImageTooLarge { .. })
        ));
    }
}
