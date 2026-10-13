//! ICO: multitamanho (§5.1, RF06).
//!
//! Um arquivo ICO é uma tabela de imagens. Cada tamanho escolhido vira uma
//! imagem PNG de 32 bits dentro do contêiner (formato aceito desde o Windows
//! Vista). A imagem de origem cabe num quadrado transparente do tamanho do
//! ícone, centralizada e sem distorção.

use ::image::codecs::png::PngEncoder;
use ::image::{DynamicImage, RgbaImage};

use super::internal;
use crate::error::CoreError;
use crate::image::options::ConvertOptions;
use crate::image::raster::Raster;
use crate::image::resize::resize_rgba8;

pub fn encode(raster: &Raster, options: &ConvertOptions) -> Result<Vec<u8>, CoreError> {
    let mut sizes = options.ico_sizes.clone();
    sizes.sort_unstable();
    sizes.dedup();

    let rgba = raster.rgba8();
    let mut images = Vec::with_capacity(sizes.len());
    for size in sizes {
        let square = fit_in_square(
            &rgba,
            raster.width,
            raster.height,
            raster.has_alpha,
            u32::from(size),
        );
        let mut png = Vec::new();
        DynamicImage::ImageRgba8(
            RgbaImage::from_raw(u32::from(size), u32::from(size), square).expect("tamanho confere"),
        )
        .write_with_encoder(PngEncoder::new(&mut png))
        .map_err(internal)?;
        images.push((size, png));
    }

    // Cabeçalho (6 bytes) + uma entrada de 16 bytes por imagem + os dados.
    let mut out = Vec::new();
    out.extend_from_slice(&[0, 0, 1, 0]); // reservado, tipo 1 = ícone
    out.extend_from_slice(&(images.len() as u16).to_le_bytes());
    let mut offset = 6 + 16 * images.len() as u32;
    for (size, png) in &images {
        // Largura e altura de 256 são gravadas como 0.
        let dim = if *size >= 256 { 0 } else { *size as u8 };
        out.extend_from_slice(&[dim, dim, 0, 0]); // dimensões, paleta, reservado
        out.extend_from_slice(&1u16.to_le_bytes()); // planos
        out.extend_from_slice(&32u16.to_le_bytes()); // bits por pixel
        out.extend_from_slice(&(png.len() as u32).to_le_bytes());
        out.extend_from_slice(&offset.to_le_bytes());
        offset += png.len() as u32;
    }
    for (_, png) in images {
        out.extend_from_slice(&png);
    }
    Ok(out)
}

/// Reduz (ou amplia) a imagem para caber num quadrado `size`×`size`,
/// centralizada sobre fundo transparente.
fn fit_in_square(rgba: &[u8], w: u32, h: u32, has_alpha: bool, size: u32) -> Vec<u8> {
    let scale = f64::from(size) / f64::from(w.max(h));
    let new_w = ((f64::from(w) * scale).round() as u32).clamp(1, size);
    let new_h = ((f64::from(h) * scale).round() as u32).clamp(1, size);
    let resized = resize_rgba8(rgba, w, h, new_w, new_h, has_alpha);

    let mut canvas = vec![0u8; (size * size * 4) as usize];
    let (off_x, off_y) = ((size - new_w) / 2, (size - new_h) / 2);
    for y in 0..new_h {
        let src = (y * new_w * 4) as usize;
        let dst = (((y + off_y) * size + off_x) * 4) as usize;
        canvas[dst..dst + (new_w * 4) as usize]
            .copy_from_slice(&resized[src..src + (new_w * 4) as usize]);
    }
    canvas
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::detect::ImageFormat;
    use crate::image::raster::Samples;

    #[test]
    fn contêiner_tem_uma_entrada_por_tamanho_em_ordem() {
        let r = Raster::from_samples(40, 40, Samples::U8(vec![200; 40 * 40 * 4]));
        let opts = ConvertOptions {
            ico_sizes: vec![256, 16, 32, 16],
            ..ConvertOptions::new(ImageFormat::Ico)
        };
        let bytes = encode(&r, &opts).unwrap();
        assert_eq!(&bytes[0..4], &[0, 0, 1, 0]);
        assert_eq!(
            u16::from_le_bytes([bytes[4], bytes[5]]),
            3,
            "duplicata removida"
        );
        assert_eq!(&bytes[6..8], &[16, 16]);
        assert_eq!(&bytes[22..24], &[32, 32]);
        assert_eq!(&bytes[38..40], &[0, 0], "256 grava 0");
    }

    #[test]
    fn imagem_nao_quadrada_e_centralizada_sem_distorcer() {
        // 8×2 opaca vira 16×16: ocupa 16×4 no meio (linhas 6 a 9).
        let r = Raster::from_samples(8, 2, Samples::U8([10, 20, 30, 255].repeat(16)));
        let sq = fit_in_square(&r.rgba8(), 8, 2, false, 16);
        let px = |x: usize, y: usize| &sq[(y * 16 + x) * 4..(y * 16 + x) * 4 + 4];
        assert_eq!(px(8, 0)[3], 0, "acima: transparente");
        assert_eq!(px(8, 7), &[10, 20, 30, 255], "no meio: a imagem");
        assert_eq!(px(8, 15)[3], 0, "abaixo: transparente");
    }
}
