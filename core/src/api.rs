//! Funções e tipos expostos ao Flutter pelo flutter_rust_bridge.
//!
//! Este é o único arquivo do núcleo que conhece a ponte. Ele traduz os tipos
//! internos para tipos simples (enums, structs, `Vec<u8>`) que o gerador
//! transforma em Dart, e captura qualquer panic para que uma falha do núcleo
//! vire um erro do item (`E-199`) em vez de derrubar o app (RNF12).
//!
//! No Dart, toda função daqui é assíncrona: o FRB a executa fora da thread
//! da interface (thread nativa no desktop, Web Worker na web).

use std::panic::{self, AssertUnwindSafe};

use flutter_rust_bridge::frb;

use crate::detect::{self, ImageFormat};
use crate::error::CoreError;
use crate::image::{self, ConvertOptions, SvgSize, TiffCompression};

/// Formato de imagem, como visto pela interface (§5.1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileFormat {
    Png,
    Jpeg,
    Webp,
    Gif,
    Bmp,
    Tiff,
    Ico,
    Svg,
}

/// Compressão do TIFF (RF06).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TiffCompressionSetting {
    None,
    Lzw,
    Deflate,
}

/// Como escolher o tamanho da rasterização de um SVG (RF06).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SvgSizeMode {
    /// O tamanho declarado no SVG (1024 px de largura se não houver).
    Declared,
    /// Largura e/ou altura em px, 1–16 384 (`width` e `height`).
    Pixels,
    /// Escala em porcentagem, 10 (0,1×) a 2000 (20×) (`scale_percent`).
    Scale,
}

/// Tamanho da rasterização de um SVG. Só os campos do `mode` escolhido valem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SvgSizeSetting {
    pub mode: SvgSizeMode,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub scale_percent: u32,
}

/// As opções de uma conversão (RF06). Os padrões vêm de [`default_settings`];
/// opções que não se aplicam ao destino são ignoradas.
#[derive(Debug, Clone)]
pub struct ConvertSettings {
    pub target: FileFormat,
    /// 1–100; JPG e WebP com perdas.
    pub quality: u8,
    /// WebP sem perdas em vez de com perdas.
    pub webp_lossless: bool,
    /// Cor de fundo no formato 0xRRGGBB, para destinos sem transparência.
    pub background: u32,
    /// Tamanhos do ICO.
    pub ico_sizes: Vec<u16>,
    pub tiff_compression: TiffCompressionSetting,
    /// Só vale para entrada SVG.
    pub svg_size: SvgSizeSetting,
}

/// Imagem convertida.
#[derive(Debug)]
pub struct ConvertedImage {
    pub bytes: Vec<u8>,
    pub source: FileFormat,
    pub target: FileFormat,
    /// Extensão da saída sem o ponto (`jpg`), conforme RN15.
    pub output_extension: String,
    /// Códigos dos avisos (`E-108`, `E-109`, `E-110`).
    pub warnings: Vec<String>,
}

/// Par nome → valor usado para preencher a mensagem traduzida do erro.
#[derive(Debug)]
pub struct ErrorParam {
    pub key: String,
    pub value: String,
}

/// Erro devolvido ao Dart. A interface usa `code` (`E-105`…) para a mensagem
/// em linguagem simples; `detail` é técnico e vai só para o relatório.
#[derive(Debug)]
pub struct MorsaError {
    pub code: String,
    pub params: Vec<ErrorParam>,
    pub detail: String,
}

/// Chamada uma vez pelo FRB ao iniciar o app.
#[frb(init)]
pub fn init_app() {
    flutter_rust_bridge::setup_default_user_utils();
}

/// Qualidade padrão do JPG e do WebP, para a interface não duplicar o valor.
#[frb(sync)]
pub fn default_quality() -> u8 {
    image::DEFAULT_QUALITY
}

/// Opções padrão do RF06 para um destino: qualidade 85, fundo branco, ICO com
/// 16, 32, 48 e 256, TIFF com LZW. É a única fonte desses valores.
#[frb(sync)]
pub fn default_settings(target: FileFormat) -> ConvertSettings {
    let d = ConvertOptions::new(target.into());
    ConvertSettings {
        target,
        quality: d.quality,
        webp_lossless: d.webp_lossless,
        background: (u32::from(d.background[0]) << 16)
            | (u32::from(d.background[1]) << 8)
            | u32::from(d.background[2]),
        ico_sizes: d.ico_sizes,
        tiff_compression: TiffCompressionSetting::Lzw,
        svg_size: SvgSizeSetting {
            mode: SvgSizeMode::Declared,
            width: None,
            height: None,
            scale_percent: 100,
        },
    }
}

/// Tamanhos de ícone que o ICO aceita (RF06).
#[frb(sync)]
pub fn ico_sizes() -> Vec<u16> {
    image::ICO_SIZES.to_vec()
}

/// Quantos bytes do início do arquivo `detect_format` precisa (64 KB, por
/// causa do SVG, que é texto; os formatos binários usam só os primeiros 4 KB).
#[frb(sync)]
pub fn detect_header_len() -> u32 {
    detect::SVG_HEADER_LEN as u32
}

/// Extensão da saída em minúsculas, sem o ponto (RN15).
#[frb(sync)]
pub fn output_extension(format: FileFormat) -> String {
    ImageFormat::from(format).extension().to_string()
}

/// Destinos válidos para uma entrada: a matriz de conversão (§5.2, RN01) é a
/// única fonte de verdade, e a interface só oferece o que está aqui (RF03).
#[frb(sync)]
pub fn valid_targets(source: FileFormat) -> Vec<FileFormat> {
    let source = ImageFormat::from(source);
    ImageFormat::ALL
        .into_iter()
        .filter(|t| image::check_conversion(source, *t).is_ok())
        .map(FileFormat::from)
        .collect()
}

/// Identifica o formato real pelo início do arquivo (RF02).
pub fn detect_format(header: Vec<u8>) -> Result<FileFormat, MorsaError> {
    guard(|| detect::detect(&header).map(FileFormat::from))
}

/// Converte uma imagem. Recebe os bytes do original e devolve um arquivo novo.
pub fn convert_image(
    input: Vec<u8>,
    settings: ConvertSettings,
) -> Result<ConvertedImage, MorsaError> {
    guard(|| {
        let options = settings.into_options()?;
        let out = image::convert(&input, &options)?;
        Ok(ConvertedImage {
            output_extension: out.target.extension().to_string(),
            bytes: out.bytes,
            source: out.source.into(),
            target: out.target.into(),
            warnings: out.warnings.iter().map(|w| w.code().to_string()).collect(),
        })
    })
}

impl ConvertSettings {
    fn into_options(self) -> Result<ConvertOptions, CoreError> {
        if self.background > 0xFF_FFFF {
            return Err(CoreError::InvalidOption {
                option: "background".into(),
                min: 0,
                max: 0xFF_FFFF,
                value: self.background,
            });
        }
        Ok(ConvertOptions {
            target: self.target.into(),
            quality: self.quality,
            webp_lossless: self.webp_lossless,
            background: [
                (self.background >> 16) as u8,
                (self.background >> 8) as u8,
                self.background as u8,
            ],
            ico_sizes: self.ico_sizes,
            tiff_compression: match self.tiff_compression {
                TiffCompressionSetting::None => TiffCompression::None,
                TiffCompressionSetting::Lzw => TiffCompression::Lzw,
                TiffCompressionSetting::Deflate => TiffCompression::Deflate,
            },
            svg_size: match self.svg_size.mode {
                SvgSizeMode::Declared => SvgSize::Declared,
                SvgSizeMode::Pixels => SvgSize::Pixels {
                    width: self.svg_size.width,
                    height: self.svg_size.height,
                },
                SvgSizeMode::Scale => SvgSize::ScalePercent(self.svg_size.scale_percent),
            },
        })
    }
}

/// Executa `work` capturando panics. Um panic vira `E-199`.
///
/// `AssertUnwindSafe` diz ao compilador que não há estado compartilhado que
/// possa ficar inconsistente depois de um panic: cada chamada só usa os
/// próprios dados.
///
/// Na web, o WASM é compilado com `panic = abort` e não dá para capturar;
/// lá o FRB devolve a falha como exceção e o Dart a trata como `E-199`.
fn guard<T>(work: impl FnOnce() -> Result<T, CoreError>) -> Result<T, MorsaError> {
    match panic::catch_unwind(AssertUnwindSafe(work)) {
        Ok(result) => result.map_err(MorsaError::from),
        Err(payload) => {
            let detail = payload
                .downcast_ref::<&str>()
                .map(|s| s.to_string())
                .or_else(|| payload.downcast_ref::<String>().cloned())
                .unwrap_or_else(|| "panic".to_string());
            Err(MorsaError::from(CoreError::Internal { detail }))
        }
    }
}

impl From<CoreError> for MorsaError {
    fn from(error: CoreError) -> Self {
        MorsaError {
            code: error.code().to_string(),
            params: error
                .params()
                .into_iter()
                .map(|(key, value)| ErrorParam { key, value })
                .collect(),
            detail: error.to_string(),
        }
    }
}

/// Liga os dois enums pela mesma lista de pares, para um não ficar sem o outro.
macro_rules! format_pairs {
    ($($core:ident <-> $api:ident),+ $(,)?) => {
        impl From<ImageFormat> for FileFormat {
            fn from(format: ImageFormat) -> Self {
                match format { $(ImageFormat::$core => FileFormat::$api),+ }
            }
        }
        impl From<FileFormat> for ImageFormat {
            fn from(format: FileFormat) -> Self {
                match format { $(FileFormat::$api => ImageFormat::$core),+ }
            }
        }
    };
}

format_pairs!(
    Png <-> Png,
    Jpeg <-> Jpeg,
    WebP <-> Webp,
    Gif <-> Gif,
    Bmp <-> Bmp,
    Tiff <-> Tiff,
    Ico <-> Ico,
    Svg <-> Svg,
);

#[cfg(test)]
mod tests {
    use super::*;

    fn settings(target: FileFormat, background: u32) -> ConvertSettings {
        ConvertSettings {
            target,
            quality: 85,
            webp_lossless: false,
            background,
            ico_sizes: vec![16],
            tiff_compression: TiffCompressionSetting::Lzw,
            svg_size: default_settings(target).svg_size,
        }
    }

    #[test]
    fn panic_vira_e199() {
        let result: Result<(), MorsaError> = guard(|| panic!("boom"));
        let error = result.expect_err("deveria falhar");
        assert_eq!(error.code, "E-199");
        assert!(error.detail.contains("boom"));
    }

    #[test]
    fn destinos_validos_seguem_a_matriz() {
        // Toda entrada vai para os formatos de saída, menos para si mesma;
        // SVG nunca é destino.
        for source in ImageFormat::ALL {
            let targets = valid_targets(source.into());
            assert!(!targets.contains(&FileFormat::Svg));
            assert!(!targets.contains(&FileFormat::from(source)));
            let expected = if source == ImageFormat::Svg { 7 } else { 6 };
            assert_eq!(targets.len(), expected, "{source:?}");
        }
    }

    #[test]
    fn cor_de_fundo_invalida_e_recusada() {
        assert!(matches!(
            settings(FileFormat::Jpeg, 0x100_0000).into_options(),
            Err(CoreError::InvalidOption { .. })
        ));
    }

    #[test]
    fn cor_de_fundo_vira_rgb() {
        let options = settings(FileFormat::Jpeg, 0x12_34_56)
            .into_options()
            .unwrap();
        assert_eq!(options.background, [0x12, 0x34, 0x56]);
    }
}
