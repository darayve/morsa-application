//! Avisos: situações em que a conversão funciona, mas o resultado merece um
//! comentário ao usuário (Apêndice A, E-108 a E-110).
//!
//! Diferente de um erro, um aviso não impede a saída: ele viaja junto com o
//! arquivo convertido.

/// Um aviso, com código estável para a interface escolher a mensagem.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Warning {
    /// E-108 · animação (GIF ou WebP): só o primeiro quadro foi usado.
    FirstFrameOnly,
    /// E-109 · TIFF com várias páginas: só a primeira foi usada.
    FirstPageOnly,
    /// E-110 · o SVG apontava para imagens ou fontes externas; foram ignoradas.
    SvgExternalResources,
}

impl Warning {
    pub fn code(self) -> &'static str {
        match self {
            Warning::FirstFrameOnly => "E-108",
            Warning::FirstPageOnly => "E-109",
            Warning::SvgExternalResources => "E-110",
        }
    }
}
