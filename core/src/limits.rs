//! Limites de segurança centralizados (SEG06).
//!
//! Imagens podem declarar dimensões gigantescas num arquivo minúsculo
//! ("decompression bomb"). Por isso as dimensões são lidas do cabeçalho e
//! conferidas aqui **antes** de qualquer decodificação.

use crate::error::CoreError;

/// Máximo de pixels por imagem: 500 megapixels.
pub const MAX_PIXELS: u64 = 500_000_000;

/// Máximo por lado: 32 768 px.
pub const MAX_SIDE: u32 = 32_768;

/// Teto de memória que uma decodificação pode alocar: 4 GB no desktop,
/// 1 GB na web. `cfg(target_arch = "wasm32")` escolhe o valor na compilação.
#[cfg(not(target_arch = "wasm32"))]
pub const MAX_DECODE_ALLOC: u64 = 4 * 1024 * 1024 * 1024;
#[cfg(target_arch = "wasm32")]
pub const MAX_DECODE_ALLOC: u64 = 1024 * 1024 * 1024;

/// Recusa com `E-106` imagens acima dos limites.
pub fn check_dimensions(width: u32, height: u32) -> Result<(), CoreError> {
    let pixels = u64::from(width) * u64::from(height);
    if width > MAX_SIDE || height > MAX_SIDE || pixels > MAX_PIXELS {
        return Err(CoreError::ImageTooLarge { width, height });
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn aceita_ate_o_limite_e_recusa_acima() {
        assert!(check_dimensions(32_768, 15_000).is_ok()); // ~491 MP
        assert!(check_dimensions(32_769, 1).is_err()); // lado
        assert!(check_dimensions(25_000, 25_000).is_err()); // 625 MP
    }
}
