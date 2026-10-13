//! Decodificação: bytes de um arquivo → [`Raster`] + avisos.
//!
//! Para formatos raster o fluxo é: ler só o cabeçalho e conferir os limites
//! (SEG06) → decodificar com teto de memória → aplicar a orientação EXIF aos
//! pixels (§5.3) → converter o perfil de cor para sRGB. SVG é tratado em
//! `svg.rs`.

use std::io::Cursor;

use ::image::{DynamicImage, ImageDecoder, ImageError, ImageReader, Limits};

use super::color;
use super::options::SvgSize;
use super::raster::Raster;
use super::svg;
use crate::detect::ImageFormat;
use crate::error::CoreError;
use crate::limits;
use crate::warning::Warning;

pub struct Decoded {
    pub raster: Raster,
    pub warnings: Vec<Warning>,
}

pub fn decode(input: &[u8], format: ImageFormat, svg_size: SvgSize) -> Result<Decoded, CoreError> {
    if format == ImageFormat::Svg {
        let (raster, warnings) = svg::render(input, svg_size)?;
        return Ok(Decoded { raster, warnings });
    }
    decode_raster(input, format)
}

fn codec(format: ImageFormat) -> ::image::ImageFormat {
    match format {
        ImageFormat::Png => ::image::ImageFormat::Png,
        ImageFormat::Jpeg => ::image::ImageFormat::Jpeg,
        ImageFormat::WebP => ::image::ImageFormat::WebP,
        ImageFormat::Gif => ::image::ImageFormat::Gif,
        ImageFormat::Bmp => ::image::ImageFormat::Bmp,
        ImageFormat::Tiff => ::image::ImageFormat::Tiff,
        ImageFormat::Ico => ::image::ImageFormat::Ico,
        ImageFormat::Svg => unreachable!("SVG não passa pelo decodificador raster"),
    }
}

fn decode_raster(input: &[u8], format: ImageFormat) -> Result<Decoded, CoreError> {
    let codec = codec(format);

    // O decodificador de JPG perdoa arquivo cortado e devolve metade da
    // imagem. Aqui isso seria um resultado errado em silêncio (E-105).
    if format == ImageFormat::Jpeg && !jpeg_is_complete(input) {
        return Err(CoreError::Corrupted {
            detail: "JPG termina antes do marcador de fim (EOI)".into(),
        });
    }

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

    let to_core = |e: ImageError| match e {
        ImageError::Limits(_) => CoreError::ImageTooLarge { width, height },
        other => map_decode_error(other),
    };

    let mut decoder = reader.into_decoder().map_err(to_core)?;
    let orientation = decoder.orientation().map_err(to_core)?;
    let icc = decoder.icc_profile().unwrap_or(None);
    let mut image = DynamicImage::from_decoder(decoder).map_err(to_core)?;

    // O EXIF diz "gire para ver certo"; aplicamos nos pixels, para que a saída
    // (sem metadados) não saia deitada.
    image.apply_orientation(orientation);

    let mut raster = Raster::from_dynamic(image);
    if let Some(icc) = icc {
        color::to_srgb(&mut raster, &icc);
        // A conversão de cor pode ter mexido nos marcadores (ex.: cinza).
        raster = Raster::from_samples(raster.width, raster.height, raster.samples);
    }

    let mut warnings = Vec::new();
    match format {
        ImageFormat::Gif if gif_has_more_frames(input) => warnings.push(Warning::FirstFrameOnly),
        ImageFormat::WebP if webp_is_animated(input) => warnings.push(Warning::FirstFrameOnly),
        ImageFormat::Tiff if tiff_has_more_pages(input) => warnings.push(Warning::FirstPageOnly),
        _ => {}
    }
    Ok(Decoded { raster, warnings })
}

/// Traduz os erros do crate `image` para os códigos do Morsa.
pub(super) fn map_decode_error(error: ImageError) -> CoreError {
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

/// O JPG chega até o marcador de fim (EOI)? Percorre a estrutura: cada
/// segmento tem tamanho; depois do SOS vêm os dados da imagem, onde todo `FF`
/// é seguido de `00` (byte literal) ou de um marcador de reinício (`D0`–`D7`);
/// qualquer outro marcador encerra esse trecho.
fn jpeg_is_complete(data: &[u8]) -> bool {
    let mut i = 2; // pula o SOI
    while i < data.len() {
        // Um marcador é FF, bytes de preenchimento FF opcionais e o código.
        if data[i] != 0xFF {
            return false;
        }
        while data.get(i + 1) == Some(&0xFF) {
            i += 1;
        }
        let Some(&marker) = data.get(i + 1) else {
            return false;
        };
        i += 2;
        match marker {
            0xD9 => return true,            // EOI
            0x01 | 0xD0..=0xD8 => continue, // sem tamanho
            _ => {}
        }
        let Some(len) = data
            .get(i..i + 2)
            .map(|b| usize::from(u16::from_be_bytes([b[0], b[1]])))
        else {
            return false;
        };
        if len < 2 || i + len > data.len() {
            return false;
        }
        i += len;
        if marker == 0xDA {
            // Dados da imagem: segue até o próximo marcador de verdade.
            loop {
                match data.get(i) {
                    None => return false,
                    Some(0xFF) => match data.get(i + 1) {
                        None => return false,
                        Some(0x00) | Some(0xD0..=0xD7) => i += 2,
                        Some(_) => break,
                    },
                    Some(_) => i += 1,
                }
            }
        }
    }
    false
}

/// GIF com mais de um quadro? Lê só os cabeçalhos dos quadros.
fn gif_has_more_frames(input: &[u8]) -> bool {
    let Ok(mut decoder) = ::gif::DecodeOptions::new().read_info(Cursor::new(input)) else {
        return false;
    };
    let mut frames = 0;
    // `read_next_frame_info` pula os dados do quadro anterior sozinho.
    while let Ok(Some(_)) = decoder.next_frame_info() {
        frames += 1;
        if frames > 1 {
            return true;
        }
    }
    false
}

/// WebP animado? O bit 1 das flags do chunk VP8X (RFC 9649 §2.7).
fn webp_is_animated(input: &[u8]) -> bool {
    input.len() > 20 && &input[12..16] == b"VP8X" && input[20] & 0b10 != 0
}

/// TIFF com mais de um diretório de imagem (página)?
fn tiff_has_more_pages(input: &[u8]) -> bool {
    ::tiff::decoder::Decoder::new(Cursor::new(input))
        .map(|d| d.more_images())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn jpeg_completo_e_cortado() {
        let jpg = include_bytes!("../../../corpus/image/jpg/tipico.jpg");
        assert!(jpeg_is_complete(jpg));
        // Cortar em qualquer ponto antes do EOI torna o arquivo incompleto.
        for cut in [3, 20, jpg.len() / 3, jpg.len() / 2, jpg.len() - 3] {
            assert!(!jpeg_is_complete(&jpg[..cut]), "corte em {cut}");
        }
        // Lixo depois do EOI não faz mal.
        let mut with_tail = jpg.to_vec();
        with_tail.extend_from_slice(b"dados extras");
        assert!(jpeg_is_complete(&with_tail));
    }

    #[test]
    fn jpeg_com_exif_e_miniatura_nao_engana() {
        // O EXIF dos testes de orientação não tem miniatura, mas o APP1 é um
        // segmento com tamanho: a verificação pula por ele corretamente.
        let jpg = include_bytes!("../../../corpus/image/jpg/exif-6.jpg");
        assert!(jpeg_is_complete(jpg));
        assert!(!jpeg_is_complete(&jpg[..jpg.len() / 2]));
    }
}
