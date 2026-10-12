//! Erros tipados do núcleo.
//!
//! Cada variante corresponde a um código estável do Apêndice A dos requisitos
//! (`E-101`, `E-102`…). A interface usa o código para escolher a mensagem
//! traduzida; o texto técnico (`Display`) serve só para logs e relatórios.

use std::fmt;

/// Tudo o que pode dar errado ao processar um arquivo.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CoreError {
    /// E-101 · conversão fora da matriz de formatos (RN01).
    UnsupportedConversion { from: String, to: String },
    /// E-102 · os primeiros bytes não correspondem a nenhum formato conhecido.
    UnknownFormat,
    /// E-103 · arquivo de 0 bytes.
    EmptyFile,
    /// E-104 · opção fora da faixa permitida.
    InvalidOption {
        option: String,
        min: u32,
        max: u32,
        value: u32,
    },
    /// E-105 · arquivo truncado ou corrompido.
    Corrupted { detail: String },
    /// E-106 · dimensões acima do limite de segurança (SEG06).
    ImageTooLarge { width: u32, height: u32 },
    /// E-199 · erro inesperado (inclui panic capturado na fronteira da ponte).
    Internal { detail: String },
}

impl CoreError {
    /// Código estável exibido no relatório e usado para buscar a mensagem.
    pub fn code(&self) -> &'static str {
        match self {
            CoreError::UnsupportedConversion { .. } => "E-101",
            CoreError::UnknownFormat => "E-102",
            CoreError::EmptyFile => "E-103",
            CoreError::InvalidOption { .. } => "E-104",
            CoreError::Corrupted { .. } => "E-105",
            CoreError::ImageTooLarge { .. } => "E-106",
            CoreError::Internal { .. } => "E-199",
        }
    }

    /// Parâmetros (nome → valor) que a mensagem traduzida pode interpolar,
    /// como `{largura}` e `{altura}` no E-106.
    pub fn params(&self) -> Vec<(String, String)> {
        match self {
            CoreError::UnsupportedConversion { from, to } => {
                vec![("from".into(), from.clone()), ("to".into(), to.clone())]
            }
            CoreError::InvalidOption {
                option,
                min,
                max,
                value,
            } => vec![
                ("option".into(), option.clone()),
                ("min".into(), min.to_string()),
                ("max".into(), max.to_string()),
                ("value".into(), value.to_string()),
            ],
            CoreError::ImageTooLarge { width, height } => vec![
                ("width".into(), width.to_string()),
                ("height".into(), height.to_string()),
            ],
            _ => Vec::new(),
        }
    }
}

impl fmt::Display for CoreError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CoreError::UnsupportedConversion { from, to } => {
                write!(f, "{}: conversão {from} → {to} fora da matriz", self.code())
            }
            CoreError::UnknownFormat => write!(f, "{}: formato não reconhecido", self.code()),
            CoreError::EmptyFile => write!(f, "{}: arquivo vazio", self.code()),
            CoreError::InvalidOption {
                option,
                min,
                max,
                value,
            } => write!(
                f,
                "{}: opção {option}={value} fora da faixa {min}–{max}",
                self.code()
            ),
            CoreError::Corrupted { detail } => {
                write!(f, "{}: arquivo corrompido ({detail})", self.code())
            }
            CoreError::ImageTooLarge { width, height } => {
                write!(
                    f,
                    "{}: imagem grande demais ({width} × {height})",
                    self.code()
                )
            }
            CoreError::Internal { detail } => write!(f, "{}: erro interno ({detail})", self.code()),
        }
    }
}

impl std::error::Error for CoreError {}
