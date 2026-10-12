//! Detecção de formato pelo conteúdo (RF02, RN14, Apêndice D).
//!
//! O formato real de um arquivo é decidido pelos primeiros bytes ("magic
//! bytes"), nunca pela extensão. No Marco 1 só PNG e JPG são reconhecidos;
//! os demais formatos da matriz entram no Marco 3.

use crate::error::CoreError;

/// Quantos bytes do início do arquivo a detecção precisa ler (RF02: até 4 KB).
pub const HEADER_LEN: usize = 4096;

/// Formatos de imagem que o núcleo sabe ler ou gravar.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ImageFormat {
    Png,
    Jpeg,
}

impl ImageFormat {
    /// Nome curto exibido na interface (`PNG → JPG`).
    pub fn label(self) -> &'static str {
        match self {
            ImageFormat::Png => "PNG",
            ImageFormat::Jpeg => "JPG",
        }
    }

    /// Extensão padrão da saída, sempre minúscula (RN15).
    pub fn extension(self) -> &'static str {
        match self {
            ImageFormat::Png => "png",
            ImageFormat::Jpeg => "jpg",
        }
    }
}

const PNG_MAGIC: &[u8] = &[0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
const JPEG_MAGIC: &[u8] = &[0xFF, 0xD8, 0xFF];

/// Identifica o formato a partir do início do arquivo.
///
/// Retorna `E-103` para arquivo vazio e `E-102` quando nenhuma assinatura
/// conhecida aparece.
pub fn detect(header: &[u8]) -> Result<ImageFormat, CoreError> {
    if header.is_empty() {
        return Err(CoreError::EmptyFile);
    }
    if header.starts_with(PNG_MAGIC) {
        Ok(ImageFormat::Png)
    } else if header.starts_with(JPEG_MAGIC) {
        Ok(ImageFormat::Jpeg)
    } else {
        Err(CoreError::UnknownFormat)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn reconhece_png_e_jpg() {
        assert_eq!(detect(PNG_MAGIC), Ok(ImageFormat::Png));
        assert_eq!(detect(&[0xFF, 0xD8, 0xFF, 0xE0]), Ok(ImageFormat::Jpeg));
    }

    #[test]
    fn vazio_e_desconhecido() {
        assert_eq!(detect(&[]), Err(CoreError::EmptyFile));
        assert_eq!(detect(b"GIF89a"), Err(CoreError::UnknownFormat));
        // Assinatura cortada no meio não conta.
        assert_eq!(detect(&PNG_MAGIC[..4]), Err(CoreError::UnknownFormat));
    }
}
