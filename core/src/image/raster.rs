//! Imagem decodificada, num formato único para todos os codificadores.
//!
//! Cada decodificador entrega pixels de um jeito (cinza, RGB, 16 bits, ponto
//! flutuante…). Aqui tudo vira RGBA intercalado, com 8 ou 16 bits por canal, e
//! duas marcas que dizem o que dá para economizar na saída: se há
//! transparência e se a imagem é cinza.

use std::borrow::Cow;

use ::image::{DynamicImage, ImageBuffer};

/// Amostras RGBA intercaladas (R, G, B, A, R, G, B, A…).
#[derive(Debug, Clone)]
pub enum Samples {
    U8(Vec<u8>),
    U16(Vec<u16>),
}

#[derive(Debug, Clone)]
pub struct Raster {
    pub width: u32,
    pub height: u32,
    pub samples: Samples,
    /// Algum pixel tem alfa abaixo do máximo.
    pub has_alpha: bool,
    /// Todos os pixels têm R = G = B.
    pub gray: bool,
}

impl Raster {
    /// Normaliza qualquer imagem do crate `image`. Imagens de 16 bits ficam em
    /// 16 bits; ponto flutuante vira 16 bits; o resto vira 8 bits.
    pub fn from_dynamic(image: DynamicImage) -> Raster {
        let (width, height) = (image.width(), image.height());
        let samples = match image {
            DynamicImage::ImageRgba8(b) => Samples::U8(b.into_raw()),
            DynamicImage::ImageLuma16(_)
            | DynamicImage::ImageLumaA16(_)
            | DynamicImage::ImageRgb16(_)
            | DynamicImage::ImageRgba16(_)
            | DynamicImage::ImageRgb32F(_)
            | DynamicImage::ImageRgba32F(_) => Samples::U16(image.to_rgba16().into_raw()),
            other => Samples::U8(other.to_rgba8().into_raw()),
        };
        Raster::from_samples(width, height, samples)
    }

    pub fn from_samples(width: u32, height: u32, samples: Samples) -> Raster {
        let (has_alpha, gray) = match &samples {
            Samples::U8(v) => analyze(v, u8::MAX),
            Samples::U16(v) => analyze(v, u16::MAX),
        };
        Raster {
            width,
            height,
            samples,
            has_alpha,
            gray,
        }
    }

    /// RGBA de 8 bits. Em imagens de 16 bits, reduz com arredondamento (§5.3).
    pub fn rgba8(&self) -> Cow<'_, [u8]> {
        match &self.samples {
            Samples::U8(v) => Cow::Borrowed(v),
            Samples::U16(v) => Cow::Owned(v.iter().map(|&s| reduce_to_8(s)).collect()),
        }
    }

    /// RGB de 8 bits (3 bytes por pixel), com a transparência composta sobre
    /// `background` (§5.3: JPG e BMP não têm transparência).
    pub fn flatten_rgb8(&self, background: [u8; 3]) -> Vec<u8> {
        let rgba = self.rgba8();
        let mut out = Vec::with_capacity(rgba.len() / 4 * 3);
        for px in rgba.chunks_exact(4) {
            let alpha = u32::from(px[3]);
            for c in 0..3 {
                out.push(composite(px[c], background[c], alpha));
            }
        }
        out
    }

    /// Tons de cinza de 8 bits (1 byte por pixel), composto sobre `background`.
    /// Só faz sentido quando a imagem é cinza e o fundo também.
    pub fn flatten_gray8(&self, background: u8) -> Vec<u8> {
        let rgba = self.rgba8();
        rgba.chunks_exact(4)
            .map(|px| composite(px[0], background, u32::from(px[3])))
            .collect()
    }

    /// A menor representação do `image` que preserva o que a imagem tem:
    /// cinza ou cor, com ou sem alfa, 8 ou 16 bits.
    pub fn to_dynamic(&self) -> DynamicImage {
        let (w, h) = (self.width, self.height);
        macro_rules! build {
            ($variant:ident, $px:ident, $n:expr, $pick:expr) => {{
                let data: Vec<_> = $px.chunks_exact(4).flat_map($pick).collect();
                debug_assert_eq!(data.len(), $px.len() / 4 * $n);
                DynamicImage::$variant(
                    ImageBuffer::from_raw(w, h, data).expect("tamanho do buffer confere"),
                )
            }};
        }
        match (&self.samples, self.gray, self.has_alpha) {
            (Samples::U8(p), true, false) => build!(ImageLuma8, p, 1, |c| [c[0]]),
            (Samples::U8(p), true, true) => build!(ImageLumaA8, p, 2, |c| [c[0], c[3]]),
            (Samples::U8(p), false, false) => build!(ImageRgb8, p, 3, |c| [c[0], c[1], c[2]]),
            (Samples::U8(p), false, true) => build!(ImageRgba8, p, 4, |c| [c[0], c[1], c[2], c[3]]),
            (Samples::U16(p), true, false) => build!(ImageLuma16, p, 1, |c| [c[0]]),
            (Samples::U16(p), true, true) => build!(ImageLumaA16, p, 2, |c| [c[0], c[3]]),
            (Samples::U16(p), false, false) => build!(ImageRgb16, p, 3, |c| [c[0], c[1], c[2]]),
            (Samples::U16(p), false, true) => {
                build!(ImageRgba16, p, 4, |c| [c[0], c[1], c[2], c[3]])
            }
        }
    }
}

/// 16 → 8 bits com arredondamento: round(v × 255 / 65535).
pub fn reduce_to_8(v: u16) -> u8 {
    ((u32::from(v) * 255 + 32_767) / 65_535) as u8
}

/// Mistura linear sobre o fundo: cor × alfa + fundo × (1 − alfa), arredondada.
fn composite(color: u8, background: u8, alpha: u32) -> u8 {
    ((u32::from(color) * alpha + u32::from(background) * (255 - alpha) + 127) / 255) as u8
}

/// (há transparência, é cinza) numa lista RGBA.
fn analyze<T: Copy + PartialEq>(rgba: &[T], max: T) -> (bool, bool) {
    let (mut has_alpha, mut gray) = (false, true);
    for px in rgba.chunks_exact(4) {
        has_alpha |= px[3] != max;
        gray &= px[0] == px[1] && px[1] == px[2];
        if has_alpha && !gray {
            break;
        }
    }
    (has_alpha, gray)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn raster(px: &[[u8; 4]]) -> Raster {
        let data = px.iter().flatten().copied().collect();
        Raster::from_samples(px.len() as u32, 1, Samples::U8(data))
    }

    #[test]
    fn detecta_alfa_e_cinza() {
        let r = raster(&[[1, 1, 1, 255], [9, 9, 9, 255]]);
        assert!(r.gray && !r.has_alpha);
        let r = raster(&[[1, 2, 1, 255]]);
        assert!(!r.gray && !r.has_alpha);
        let r = raster(&[[5, 5, 5, 254]]);
        assert!(r.gray && r.has_alpha);
    }

    #[test]
    fn reduz_16_para_8_com_arredondamento() {
        assert_eq!(reduce_to_8(0), 0);
        assert_eq!(reduce_to_8(65_535), 255);
        assert_eq!(reduce_to_8(32_768), 128);
        // 257 é exatamente 1 em 8 bits; 128 fica abaixo da metade.
        assert_eq!(reduce_to_8(257), 1);
        assert_eq!(reduce_to_8(128), 0);
        assert_eq!(reduce_to_8(129), 1);
    }

    #[test]
    fn composicao_sobre_o_fundo() {
        let r = raster(&[[10, 20, 30, 255], [10, 20, 30, 0], [0, 0, 0, 128]]);
        let rgb = r.flatten_rgb8([255, 255, 255]);
        assert_eq!(&rgb[0..3], &[10, 20, 30]); // opaco: mantém
        assert_eq!(&rgb[3..6], &[255, 255, 255]); // transparente: fundo
        assert_eq!(&rgb[6..9], &[127, 127, 127]); // meio a meio
        let colored = r.flatten_rgb8([255, 0, 0]);
        assert_eq!(&colored[3..6], &[255, 0, 0]);
    }

    #[test]
    fn to_dynamic_escolhe_a_menor_representacao() {
        let gray = raster(&[[7, 7, 7, 255]]).to_dynamic();
        assert!(matches!(gray, DynamicImage::ImageLuma8(_)));
        let rgba = raster(&[[7, 8, 9, 100]]).to_dynamic();
        assert!(matches!(rgba, DynamicImage::ImageRgba8(_)));
        let wide = Raster::from_samples(1, 1, Samples::U16(vec![1000, 2000, 3000, 65_535]));
        assert!(matches!(wide.to_dynamic(), DynamicImage::ImageRgb16(_)));
    }
}
