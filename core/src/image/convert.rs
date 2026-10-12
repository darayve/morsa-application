//! Conversão de uma imagem de um formato para outro.
//!
//! Fluxo: detectar o formato real → conferir a matriz e as opções → ler só
//! as dimensões e conferir os limites → decodificar → (se o destino não tem
//! transparência) compor sobre o branco → codificar.

use std::io::Cursor;

use ::image::codecs::jpeg::JpegEncoder;
use ::image::codecs::png::PngEncoder;
use ::image::{DynamicImage, ImageError, ImageReader, Limits, RgbImage};

use crate::detect::{self, ImageFormat};
use crate::error::CoreError;
use crate::limits;

/// Qualidade padrão do JPG (RF06).
pub const DEFAULT_JPEG_QUALITY: u8 = 85;

/// Opções da conversão. Cresce no Marco 3 (fundo, metadados, edição…).
#[derive(Debug, Clone, Copy)]
pub struct ConvertOptions {
    pub target: ImageFormat,
    /// 1–100; só usada quando o destino é JPG.
    pub quality: u8,
}

/// Resultado: os bytes do arquivo novo e os formatos envolvidos.
#[derive(Debug, Clone)]
pub struct ConvertOutput {
    pub bytes: Vec<u8>,
    pub source: ImageFormat,
    pub target: ImageFormat,
}

/// Converte `input` conforme `options`.
///
/// `input` é emprestado (`&[u8]`): o Rust garante que esta função não pode
/// modificar os bytes do arquivo original (RN02). A saída é sempre um vetor novo.
pub fn convert(input: &[u8], options: ConvertOptions) -> Result<ConvertOutput, CoreError> {
    let source = detect::detect(input)?;
    super::check_conversion(source, options.target)?;
    validate_quality(options.quality)?;

    let decoded = decode(input, source)?;

    let bytes = match options.target {
        ImageFormat::Jpeg => encode_jpeg(&decoded, options.quality)?,
        ImageFormat::Png => encode_png(&decoded)?,
    };

    Ok(ConvertOutput {
        bytes,
        source,
        target: options.target,
    })
}

fn validate_quality(quality: u8) -> Result<(), CoreError> {
    if !(1..=100).contains(&quality) {
        return Err(CoreError::InvalidOption {
            option: "quality".into(),
            min: 1,
            max: 100,
            value: quality.into(),
        });
    }
    Ok(())
}

/// Decodifica respeitando os limites de segurança.
fn decode(input: &[u8], format: ImageFormat) -> Result<DynamicImage, CoreError> {
    let codec = match format {
        ImageFormat::Png => ::image::ImageFormat::Png,
        ImageFormat::Jpeg => ::image::ImageFormat::Jpeg,
    };

    // 1ª passada: só o cabeçalho, para saber as dimensões sem alocar pixels.
    let (width, height) = ImageReader::with_format(Cursor::new(input), codec)
        .into_dimensions()
        .map_err(map_decode_error)?;
    limits::check_dimensions(width, height)?;

    // 2ª passada: decodificação completa, com teto de memória como segunda barreira.
    let mut reader = ImageReader::with_format(Cursor::new(input), codec);
    let mut decode_limits = Limits::default();
    decode_limits.max_image_width = Some(limits::MAX_SIDE);
    decode_limits.max_image_height = Some(limits::MAX_SIDE);
    decode_limits.max_alloc = Some(limits::MAX_DECODE_ALLOC);
    reader.limits(decode_limits);

    reader.decode().map_err(|e| match e {
        ImageError::Limits(_) => CoreError::ImageTooLarge { width, height },
        other => map_decode_error(other),
    })
}

/// Traduz os erros do crate `image` para os códigos do Morsa.
fn map_decode_error(error: ImageError) -> CoreError {
    match error {
        // Conteúdo inválido ou arquivo que acaba antes da hora: E-105.
        ImageError::Decoding(e) => CoreError::Corrupted {
            detail: e.to_string(),
        },
        ImageError::IoError(e) => CoreError::Corrupted {
            detail: e.to_string(),
        },
        // Variante do formato que o decodificador não implementa: E-102.
        ImageError::Unsupported(_) => CoreError::UnknownFormat,
        other => CoreError::Internal {
            detail: other.to_string(),
        },
    }
}

/// JPG não tem transparência: compõe cada pixel sobre o branco (§5.3).
/// Imagens de 16 bits são reduzidas para 8 bits com arredondamento por `to_rgba8`.
fn flatten_on_white(image: &DynamicImage) -> RgbImage {
    let rgba = image.to_rgba8();
    let mut rgb = RgbImage::new(rgba.width(), rgba.height());
    for (out, px) in rgb.pixels_mut().zip(rgba.pixels()) {
        let alpha = u32::from(px[3]);
        for c in 0..3 {
            // Mistura linear: cor × alfa + branco × (1 − alfa), com arredondamento.
            let value = (u32::from(px[c]) * alpha + 255 * (255 - alpha) + 127) / 255;
            out[c] = value as u8;
        }
    }
    rgb
}

fn encode_jpeg(image: &DynamicImage, quality: u8) -> Result<Vec<u8>, CoreError> {
    let rgb = flatten_on_white(image);
    let mut out = Vec::new();
    JpegEncoder::new_with_quality(&mut out, quality)
        .encode_image(&rgb)
        .map_err(|e| CoreError::Internal {
            detail: e.to_string(),
        })?;
    Ok(out)
}

fn encode_png(image: &DynamicImage) -> Result<Vec<u8>, CoreError> {
    let mut out = Vec::new();
    image
        .write_with_encoder(PngEncoder::new(&mut out))
        .map_err(|e| CoreError::Internal {
            detail: e.to_string(),
        })?;
    Ok(out)
}
