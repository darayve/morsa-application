//! Utilitários dos testes de integração de imagens.
//!
//! Cada teste lê arquivos reais de `corpus/`. `Original` também guarda os
//! bytes lidos para provar, no fim, que o arquivo em disco não mudou
//! (RN02, TST06).
#![allow(dead_code)] // cada arquivo de teste usa uma parte

use std::fs;
use std::path::PathBuf;

use ::image::{ImageFormat as Codec, RgbaImage};
use morsa_core::detect::ImageFormat;
use morsa_core::image::{ConvertOptions, ConvertOutput, convert};

pub fn corpus_path(relative: &str) -> PathBuf {
    // CARGO_MANIFEST_DIR é a pasta core/; o corpus fica um nível acima.
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("../corpus/image")
        .join(relative)
}

/// Lê o arquivo e vira uma testemunha que confere, ao ser descartada, que o
/// arquivo em disco continua idêntico.
pub struct Original {
    path: PathBuf,
    pub bytes: Vec<u8>,
}

impl Original {
    pub fn read(relative: &str) -> Self {
        let path = corpus_path(relative);
        let bytes = fs::read(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        Original { path, bytes }
    }
}

impl Drop for Original {
    fn drop(&mut self) {
        let after = fs::read(&self.path).unwrap();
        assert_eq!(
            after,
            self.bytes,
            "o original foi alterado: {}",
            self.path.display()
        );
    }
}

/// Converte um arquivo do corpus para `target` com as opções padrão.
pub fn convert_file(relative: &str, target: ImageFormat) -> ConvertOutput {
    convert_file_with(relative, ConvertOptions::new(target))
}

pub fn convert_file_with(relative: &str, options: ConvertOptions) -> ConvertOutput {
    let original = Original::read(relative);
    convert(&original.bytes, &options)
        .unwrap_or_else(|e| panic!("{relative} → {:?}: {e}", options.target))
}

/// Código de erro de uma conversão que precisa falhar.
pub fn error_code(relative: &str, target: ImageFormat) -> String {
    let original = Original::read(relative);
    match convert(&original.bytes, &ConvertOptions::new(target)) {
        Ok(_) => panic!("{relative} → {target:?} deveria falhar"),
        Err(e) => e.code().to_string(),
    }
}

pub fn codec(format: ImageFormat) -> Codec {
    match format {
        ImageFormat::Png => Codec::Png,
        ImageFormat::Jpeg => Codec::Jpeg,
        ImageFormat::WebP => Codec::WebP,
        ImageFormat::Gif => Codec::Gif,
        ImageFormat::Bmp => Codec::Bmp,
        ImageFormat::Tiff => Codec::Tiff,
        ImageFormat::Ico => Codec::Ico,
        ImageFormat::Svg => panic!("SVG não é formato de saída"),
    }
}

/// Decodifica o resultado com um leitor independente do núcleo (o crate
/// `image` direto, sem passar pelas regras do Morsa).
pub fn decode_output(bytes: &[u8], format: ImageFormat) -> RgbaImage {
    ::image::load_from_memory_with_format(bytes, codec(format))
        .unwrap_or_else(|e| panic!("saída {format:?} não decodifica: {e}"))
        .to_rgba8()
}

/// Decodifica um arquivo de entrada do corpus.
pub fn decode_file(relative: &str, format: ImageFormat) -> RgbaImage {
    decode_output(&Original::read(relative).bytes, format)
}

/// PSNR em dB entre dois conjuntos de canais RGB (ignora o alfa).
pub fn psnr(a: &RgbaImage, b: &RgbaImage) -> f64 {
    assert_eq!(a.dimensions(), b.dimensions());
    let mut squared = 0f64;
    for (x, y) in a.pixels().zip(b.pixels()) {
        for c in 0..3 {
            squared += (f64::from(x[c]) - f64::from(y[c])).powi(2);
        }
    }
    let mse = squared / (f64::from(a.width()) * f64::from(a.height()) * 3.0);
    if mse == 0.0 {
        return f64::INFINITY;
    }
    10.0 * (255.0f64 * 255.0 / mse).log10()
}

/// Compõe a imagem sobre o branco, como as saídas sem transparência fazem.
pub fn flatten_on_white(img: &RgbaImage) -> RgbaImage {
    let mut out = img.clone();
    for p in out.pixels_mut() {
        let a = u32::from(p[3]);
        for c in 0..3 {
            p[c] = ((u32::from(p[c]) * a + 255 * (255 - a) + 127) / 255) as u8;
        }
        p[3] = 255;
    }
    out
}

/// Diferença máxima, por canal, entre dois pixels.
pub fn channel_distance(a: [u8; 4], b: [u8; 4]) -> i32 {
    (0..3)
        .map(|c| (i32::from(a[c]) - i32::from(b[c])).abs())
        .max()
        .unwrap()
}

/// Erro médio absoluto entre as médias de cada bloco `n`×`n`. Mede o que o
/// olho vê em imagens pontilhadas (GIF), onde o erro por pixel é alto de
/// propósito, mas se compensa dentro da vizinhança.
pub fn block_mean_error(a: &RgbaImage, b: &RgbaImage, n: u32) -> f64 {
    assert_eq!(a.dimensions(), b.dimensions());
    let (mut total, mut count) = (0f64, 0f64);
    for by in (0..a.height() - n + 1).step_by(n as usize) {
        for bx in (0..a.width() - n + 1).step_by(n as usize) {
            for c in 0..3 {
                let mean = |img: &RgbaImage| {
                    let mut sum = 0f64;
                    for y in by..by + n {
                        for x in bx..bx + n {
                            sum += f64::from(img.get_pixel(x, y)[c]);
                        }
                    }
                    sum / f64::from(n * n)
                };
                total += (mean(a) - mean(b)).abs();
                count += 1.0;
            }
        }
    }
    total / count
}
