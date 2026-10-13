//! Redimensionamento com Lanczos3 (RF07).
//!
//! Cor com transparência não pode ser misturada "reta": o pixel transparente
//! tem uma cor qualquer (muitas vezes preta), que vazaria para a borda e
//! deixaria um contorno escuro. Por isso, com alfa, a conversão passa por
//! cor pré-multiplicada em ponto flutuante.

use ::image::imageops::{self, FilterType};
use ::image::{ImageBuffer, Rgba, RgbaImage};

/// Redimensiona RGBA de 8 bits de `w`×`h` para `new_w`×`new_h`.
pub fn resize_rgba8(
    src: &[u8],
    w: u32,
    h: u32,
    new_w: u32,
    new_h: u32,
    has_alpha: bool,
) -> Vec<u8> {
    if (w, h) == (new_w, new_h) {
        return src.to_vec();
    }
    if !has_alpha {
        let img = RgbaImage::from_raw(w, h, src.to_vec()).expect("tamanho confere");
        return imageops::resize(&img, new_w, new_h, FilterType::Lanczos3).into_raw();
    }

    // O `image` trata ponto flutuante como 0,0 a 1,0 (e satura em 1,0), então
    // todos os canais, alfa incluído, vão para essa escala.
    let premultiplied: Vec<f32> = src
        .chunks_exact(4)
        .flat_map(|p| {
            let a = f32::from(p[3]) / 255.0;
            [
                f32::from(p[0]) / 255.0 * a,
                f32::from(p[1]) / 255.0 * a,
                f32::from(p[2]) / 255.0 * a,
                a,
            ]
        })
        .collect();
    let img: ImageBuffer<Rgba<f32>, Vec<f32>> =
        ImageBuffer::from_raw(w, h, premultiplied).expect("tamanho confere");
    let resized = imageops::resize(&img, new_w, new_h, FilterType::Lanczos3);

    resized
        .as_raw()
        .chunks_exact(4)
        .flat_map(|p| {
            let a = p[3].clamp(0.0, 1.0);
            let k = if a > 0.0 { 255.0 / a } else { 0.0 };
            [
                (p[0] * k).round().clamp(0.0, 255.0) as u8,
                (p[1] * k).round().clamp(0.0, 255.0) as u8,
                (p[2] * k).round().clamp(0.0, 255.0) as u8,
                (a * 255.0).round() as u8,
            ]
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mesmo_tamanho_devolve_igual() {
        let px = vec![1, 2, 3, 4, 5, 6, 7, 8];
        assert_eq!(resize_rgba8(&px, 2, 1, 2, 1, true), px);
    }

    #[test]
    fn cor_solida_continua_a_mesma_ao_reduzir() {
        let px: Vec<u8> = (0..16 * 16).flat_map(|_| [200, 100, 50, 255]).collect();
        let out = resize_rgba8(&px, 16, 16, 4, 4, false);
        assert_eq!(out.len(), 4 * 4 * 4);
        assert!(out.chunks(4).all(|p| p == [200, 100, 50, 255]));
    }

    #[test]
    fn transparente_preto_nao_escurece_a_borda() {
        // Metade esquerda: vermelho opaco. Metade direita: preto transparente.
        // Sem pré-multiplicar, a coluna do meio sairia marrom-escura.
        let mut px = Vec::new();
        for _ in 0..8 {
            for x in 0..16 {
                px.extend_from_slice(if x < 8 {
                    &[255, 0, 0, 255]
                } else {
                    &[0, 0, 0, 0]
                });
            }
        }
        let out = resize_rgba8(&px, 16, 8, 8, 4, true);
        let at = |x: usize| &out[(2 * 8 + x) * 4..(2 * 8 + x) * 4 + 4];
        assert_eq!(at(0), &[255, 0, 0, 255], "miolo vermelho intacto");
        assert_eq!(at(7)[3], 0, "miolo transparente intacto");
        for p in out.chunks(4).filter(|p| p[3] > 8) {
            assert!(p[0] >= 250 && p[1] <= 4 && p[2] <= 4, "cor da borda: {p:?}");
        }
    }

    #[test]
    fn meia_transparencia_continua_meia_transparencia() {
        let px: Vec<u8> = (0..8 * 8).flat_map(|_| [10, 200, 90, 128]).collect();
        let out = resize_rgba8(&px, 8, 8, 4, 4, true);
        assert!(
            out.chunks(4).all(|p| p == [10, 200, 90, 128]),
            "{:?}",
            &out[..4]
        );
    }
}
