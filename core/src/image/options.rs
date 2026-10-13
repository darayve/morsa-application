//! Opções da conversão de imagens (RF06) e a validação das faixas (E-104).

use crate::detect::ImageFormat;
use crate::error::CoreError;

/// Qualidade padrão do JPG e do WebP com perdas (RF06).
pub const DEFAULT_QUALITY: u8 = 85;

/// Tamanhos de ícone que o ICO aceita (RF06).
pub const ICO_SIZES: [u16; 7] = [16, 24, 32, 48, 64, 128, 256];

/// Tamanhos padrão do ICO (RF06).
pub const DEFAULT_ICO_SIZES: [u16; 4] = [16, 32, 48, 256];

/// Maior lado ao rasterizar um SVG, em pixels (RF06).
pub const MAX_SVG_SIDE: u32 = 16_384;

/// Compressão do TIFF (RF06).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiffCompression {
    None,
    Lzw,
    Deflate,
}

/// Tamanho da rasterização de um SVG (RF06).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvgSize {
    /// O tamanho declarado no SVG. Sem `width` e `height`, vale 1024 px de
    /// largura (RF06).
    Declared,
    /// Largura e/ou altura em px (1–16 384). Com as duas, a imagem cabe na
    /// caixa mantendo a proporção.
    Pixels {
        width: Option<u32>,
        height: Option<u32>,
    },
    /// Escala em porcentagem sobre o tamanho declarado: 10 (0,1×) a 2000 (20×).
    ScalePercent(u32),
}

/// Tudo o que o usuário pode escolher numa conversão. Opções que não se
/// aplicam ao destino (ex.: `quality` num PNG) são simplesmente ignoradas.
#[derive(Debug, Clone, PartialEq)]
pub struct ConvertOptions {
    pub target: ImageFormat,
    /// 1–100; JPG e WebP com perdas.
    pub quality: u8,
    /// WebP sem perdas em vez de com perdas.
    pub webp_lossless: bool,
    /// Cor de fundo (R, G, B) onde o destino não tem transparência: JPG e BMP.
    pub background: [u8; 3],
    /// Tamanhos do ICO, subconjunto de [`ICO_SIZES`].
    pub ico_sizes: Vec<u16>,
    pub tiff_compression: TiffCompression,
    /// Só vale para entrada SVG.
    pub svg_size: SvgSize,
}

impl ConvertOptions {
    /// Padrões do RF06 para o destino escolhido.
    pub fn new(target: ImageFormat) -> Self {
        Self {
            target,
            quality: DEFAULT_QUALITY,
            webp_lossless: false,
            background: [255, 255, 255],
            ico_sizes: DEFAULT_ICO_SIZES.to_vec(),
            tiff_compression: TiffCompression::Lzw,
            svg_size: SvgSize::Declared,
        }
    }

    /// Confere as faixas do RF06. O núcleo valida mesmo que a interface já
    /// tenha validado (defesa em profundidade).
    pub fn validate(&self) -> Result<(), CoreError> {
        if !(1..=100).contains(&self.quality) {
            return Err(invalid("quality", 1, 100, self.quality.into()));
        }
        if self.target == ImageFormat::Ico {
            if self.ico_sizes.is_empty() {
                return Err(invalid("ico_sizes", 1, 7, 0));
            }
            if let Some(bad) = self.ico_sizes.iter().find(|s| !ICO_SIZES.contains(s)) {
                return Err(invalid("ico_sizes", 16, 256, (*bad).into()));
            }
        }
        match self.svg_size {
            SvgSize::Declared => {}
            SvgSize::Pixels { width, height } => {
                if width.is_none() && height.is_none() {
                    return Err(invalid("svg_size", 1, MAX_SVG_SIDE, 0));
                }
                for v in [width, height].into_iter().flatten() {
                    if !(1..=MAX_SVG_SIDE).contains(&v) {
                        return Err(invalid("svg_size", 1, MAX_SVG_SIDE, v));
                    }
                }
            }
            SvgSize::ScalePercent(p) => {
                if !(10..=2000).contains(&p) {
                    return Err(invalid("svg_scale", 10, 2000, p));
                }
            }
        }
        Ok(())
    }
}

fn invalid(option: &str, min: u32, max: u32, value: u32) -> CoreError {
    CoreError::InvalidOption {
        option: option.into(),
        min,
        max,
        value,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn base() -> ConvertOptions {
        ConvertOptions::new(ImageFormat::Png)
    }

    #[test]
    fn padroes_sao_validos() {
        for f in ImageFormat::ALL {
            assert_eq!(ConvertOptions::new(f).validate(), Ok(()));
        }
    }

    #[test]
    fn qualidade_fora_da_faixa() {
        for q in [0, 101, 255] {
            let o = ConvertOptions {
                quality: q,
                ..base()
            };
            assert!(matches!(
                o.validate(),
                Err(CoreError::InvalidOption { ref option, .. }) if option == "quality"
            ));
        }
    }

    #[test]
    fn tamanhos_do_ico() {
        let ico = ConvertOptions::new(ImageFormat::Ico);
        assert!(
            ConvertOptions {
                ico_sizes: vec![],
                ..ico.clone()
            }
            .validate()
            .is_err()
        );
        assert!(
            ConvertOptions {
                ico_sizes: vec![16, 20],
                ..ico.clone()
            }
            .validate()
            .is_err()
        );
        assert!(
            ConvertOptions {
                ico_sizes: vec![256, 16],
                ..ico
            }
            .validate()
            .is_ok()
        );
        // Fora do ICO, a lista é ignorada.
        assert!(
            ConvertOptions {
                ico_sizes: vec![],
                ..base()
            }
            .validate()
            .is_ok()
        );
    }

    #[test]
    fn tamanho_do_svg() {
        let ok = |s| {
            ConvertOptions {
                svg_size: s,
                ..base()
            }
            .validate()
            .is_ok()
        };
        assert!(ok(SvgSize::Pixels {
            width: Some(1),
            height: None
        }));
        assert!(ok(SvgSize::Pixels {
            width: Some(16_384),
            height: Some(10)
        }));
        assert!(!ok(SvgSize::Pixels {
            width: None,
            height: None
        }));
        assert!(!ok(SvgSize::Pixels {
            width: Some(0),
            height: None
        }));
        assert!(!ok(SvgSize::Pixels {
            width: Some(16_385),
            height: None
        }));
        assert!(ok(SvgSize::ScalePercent(10)));
        assert!(ok(SvgSize::ScalePercent(2000)));
        assert!(!ok(SvgSize::ScalePercent(9)));
        assert!(!ok(SvgSize::ScalePercent(2001)));
    }
}
