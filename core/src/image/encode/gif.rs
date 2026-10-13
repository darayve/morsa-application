//! GIF: estática, paleta de até 256 cores (§5.1, §5.3).
//!
//! - Transparência é binária: pixels com alfa abaixo de 50% viram o índice
//!   transparente; os demais ficam opacos.
//! - Até 256 cores: a paleta é exata, sem perda.
//! - Acima disso: quantização NeuQuant com pontilhamento (dithering)
//!   Floyd–Steinberg, que espalha o erro de cor entre pixels vizinhos para
//!   gradientes não virarem faixas.

use std::borrow::Cow;
use std::collections::HashMap;

use color_quant::NeuQuant;

use super::{check_side, internal};
use crate::error::CoreError;
use crate::image::raster::Raster;

/// Alfa a partir do qual o pixel é opaco (50%).
const OPAQUE_FROM: u8 = 128;

/// Quantos pixels o NeuQuant precisa ver, mais ou menos, para aprender uma boa
/// paleta. Imagens menores são lidas inteiras (fator 1, o melhor); nas maiores
/// o fator cresce para amostrar só alguns pixels, e o custo não explode.
const SAMPLE_TARGET: usize = 1_000_000;
const MAX_SAMPLE_FACTOR: usize = 30;

pub fn encode(raster: &Raster) -> Result<Vec<u8>, CoreError> {
    check_side(raster, 65_535)?;
    let rgba = raster.rgba8();
    let (w, h) = (raster.width as usize, raster.height as usize);

    let has_transparent = rgba.chunks_exact(4).any(|p| p[3] < OPAQUE_FROM);
    let (palette, indices, transparent) = quantize(&rgba, w, h, has_transparent, true);

    let mut out = Vec::new();
    {
        let mut encoder =
            ::gif::Encoder::new(&mut out, raster.width as u16, raster.height as u16, &[])
                .map_err(internal)?;
        let frame = ::gif::Frame {
            width: raster.width as u16,
            height: raster.height as u16,
            buffer: Cow::Owned(indices),
            palette: Some(palette),
            transparent,
            ..::gif::Frame::default()
        };
        encoder.write_frame(&frame).map_err(internal)?;
    }
    Ok(out)
}

/// (paleta RGB, índice de cada pixel, índice transparente).
fn quantize(
    rgba: &[u8],
    w: usize,
    h: usize,
    has_transparent: bool,
    dither: bool,
) -> (Vec<u8>, Vec<u8>, Option<u8>) {
    let max_colors = if has_transparent { 255 } else { 256 };

    // Cores distintas dos pixels opacos, até estourar o limite.
    let mut seen: HashMap<[u8; 3], u8> = HashMap::new();
    let mut palette: Vec<u8> = Vec::new();
    let mut fits = true;
    for p in rgba.chunks_exact(4).filter(|p| p[3] >= OPAQUE_FROM) {
        let key = [p[0], p[1], p[2]];
        if !seen.contains_key(&key) {
            if seen.len() == max_colors {
                fits = false;
                break;
            }
            seen.insert(key, seen.len() as u8);
            palette.extend_from_slice(&key);
        }
    }

    if fits {
        // Paleta exata.
        let transparent = has_transparent.then_some(seen.len() as u8);
        if has_transparent {
            palette.extend_from_slice(&[0, 0, 0]);
        }
        if palette.is_empty() {
            palette.extend_from_slice(&[0, 0, 0]); // imagem toda transparente
        }
        let indices = rgba
            .chunks_exact(4)
            .map(|p| match transparent {
                Some(t) if p[3] < OPAQUE_FROM => t,
                _ => seen[&[p[0], p[1], p[2]]],
            })
            .collect();
        return (palette, indices, transparent);
    }

    // Mais de 256 cores: aprende uma paleta com os pixels opacos...
    let opaque: Vec<u8> = rgba
        .chunks_exact(4)
        .filter(|p| p[3] >= OPAQUE_FROM)
        .flat_map(|p| [p[0], p[1], p[2], 255])
        .collect();
    let factor = (opaque.len() / 4 / SAMPLE_TARGET).clamp(1, MAX_SAMPLE_FACTOR) as i32;
    let nq = NeuQuant::new(factor, max_colors, &opaque);
    let mut palette = nq.color_map_rgb();
    let transparent = has_transparent.then_some((palette.len() / 3) as u8);
    if has_transparent {
        palette.extend_from_slice(&[0, 0, 0]);
    }

    // ...e pinta com pontilhamento Floyd–Steinberg. Os erros ficam em
    // unidades de 1/16 para usar só inteiros: o pixel recebe 7/16 do erro do
    // vizinho da esquerda, 3/16 do de cima à direita, 5/16 do de cima e 1/16
    // do de cima à esquerda (aqui, empurrados para a frente).
    let mut indices = vec![0u8; w * h];
    let mut current = vec![[0i32; 3]; w + 2];
    let mut next = vec![[0i32; 3]; w + 2];
    for y in 0..h {
        for x in 0..w {
            let p = &rgba[(y * w + x) * 4..(y * w + x) * 4 + 4];
            if p[3] < OPAQUE_FROM {
                indices[y * w + x] = transparent.expect("há pixel transparente");
                continue;
            }
            let carried = if dither { current[x + 1] } else { [0; 3] };
            let mut want = [0i32; 3];
            for c in 0..3 {
                want[c] = (i32::from(p[c]) + carried[c].div_euclid(16)).clamp(0, 255);
            }
            let idx = nq.index_of(&[want[0] as u8, want[1] as u8, want[2] as u8, 255]);
            let got = nq.lookup(idx).expect("índice vem do próprio NeuQuant");
            indices[y * w + x] = idx as u8;
            for c in 0..3 {
                let error = want[c] - i32::from(got[c]);
                current[x + 2][c] += error * 7;
                next[x][c] += error * 3;
                next[x + 1][c] += error * 5;
                next[x + 2][c] += error;
            }
        }
        std::mem::swap(&mut current, &mut next);
        next.iter_mut().for_each(|e| *e = [0; 3]);
    }
    (palette, indices, transparent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::image::raster::Samples;

    fn decode(bytes: &[u8]) -> (u32, u32, Vec<u8>) {
        let img = ::image::load_from_memory_with_format(bytes, ::image::ImageFormat::Gif)
            .unwrap()
            .to_rgba8();
        (img.width(), img.height(), img.into_raw())
    }

    #[test]
    fn poucas_cores_ficam_exatas_e_alfa_vira_binario() {
        // 4 pixels: vermelho opaco, azul com alfa 200 (opaco), verde com alfa
        // 127 (vira transparente) e transparente.
        let px = vec![255, 0, 0, 255, 0, 0, 255, 200, 0, 255, 0, 127, 9, 9, 9, 0];
        let r = Raster::from_samples(4, 1, Samples::U8(px));
        let (w, h, out) = decode(&encode(&r).unwrap());
        assert_eq!((w, h), (4, 1));
        assert_eq!(&out[0..4], &[255, 0, 0, 255]);
        assert_eq!(&out[4..8], &[0, 0, 255, 255]);
        assert_eq!(out[11], 0);
        assert_eq!(out[15], 0);
    }

    /// Gradiente com milhares de cores, bem mais que as 256 da paleta.
    fn gradient(w: u32, h: u32) -> Vec<u8> {
        let mut px = Vec::new();
        for y in 0..h {
            for x in 0..w {
                px.extend_from_slice(&[(x * 2) as u8, (y * 2) as u8, ((x + y) % 256) as u8, 255]);
            }
        }
        px
    }

    #[test]
    fn muitas_cores_vira_paleta_de_256() {
        let (w, h) = (128u32, 128u32);
        let r = Raster::from_samples(w, h, Samples::U8(gradient(w, h)));
        let (_, _, out) = decode(&encode(&r).unwrap());
        let colors: std::collections::HashSet<_> =
            out.chunks(4).map(|p| [p[0], p[1], p[2]]).collect();
        assert!(colors.len() <= 256, "{} cores", colors.len());
    }

    /// O pontilhamento troca erro por pixel por erro que se compensa na
    /// vizinhança: a média de cada bloco de 4×4 fica mais perto do original.
    #[test]
    fn pontilhamento_aproxima_a_media_local() {
        let (w, h) = (128usize, 128usize);
        let px = gradient(w as u32, h as u32);

        let block_error = |dither: bool| {
            let (palette, idx, _) = quantize(&px, w, h, false, dither);
            let mut total = 0f64;
            for by in (0..h).step_by(4) {
                for bx in (0..w).step_by(4) {
                    let (mut want, mut got) = ([0f64; 3], [0f64; 3]);
                    for y in by..by + 4 {
                        for x in bx..bx + 4 {
                            let i = y * w + x;
                            for c in 0..3 {
                                want[c] += f64::from(px[i * 4 + c]);
                                got[c] += f64::from(palette[usize::from(idx[i]) * 3 + c]);
                            }
                        }
                    }
                    for c in 0..3 {
                        total += ((want[c] - got[c]) / 16.0).powi(2);
                    }
                }
            }
            total
        };
        let (with, without) = (block_error(true), block_error(false));
        assert!(with < without * 0.6, "com {with:.0} vs sem {without:.0}");
    }

    #[test]
    fn imagem_toda_transparente_funciona() {
        let r = Raster::from_samples(2, 2, Samples::U8(vec![0; 16]));
        let (_, _, out) = decode(&encode(&r).unwrap());
        assert!(out.chunks(4).all(|p| p[3] == 0));
    }
}
