//! Perfil de cor: converte para sRGB (§5.3).
//!
//! Fotos podem trazer um perfil ICC (Display P3, Adobe RGB…). Se o perfil for
//! ignorado, as cores mudam ao abrir o resultado em outro programa. Por isso os
//! pixels são convertidos para sRGB, que é o que todos os formatos de saída
//! assumem quando não há perfil.
//!
//! A conversão é "melhor esforço": perfil ilegível, de cinza ou de CMYK não
//! derruba a conversão; a imagem segue como estava.

use moxcms::{ColorProfile, DataColorSpace, Layout, TransformOptions};

use super::raster::{Raster, Samples};

/// Diferença máxima, em níveis de 8 bits, para considerar que o perfil já é
/// equivalente ao sRGB e dispensar a conversão (evita arredondar à toa).
const SAME_AS_SRGB_TOLERANCE: i32 = 1;

/// Converte `raster` do perfil `icc` para sRGB. Devolve `true` se mudou pixels.
pub fn to_srgb(raster: &mut Raster, icc: &[u8]) -> bool {
    let Ok(source) = ColorProfile::new_from_slice(icc) else {
        return false;
    };
    // Só perfis RGB: o `image` já entrega cinza e CMYK como RGB.
    if source.color_space != DataColorSpace::Rgb {
        return false;
    }
    let srgb = ColorProfile::new_srgb();
    let options = TransformOptions::default();

    match &mut raster.samples {
        Samples::U8(pixels) => {
            let Ok(transform) =
                source.create_transform_8bit(Layout::Rgba, &srgb, Layout::Rgba, options)
            else {
                return false;
            };
            if is_equivalent_to_srgb(|src, dst| transform.transform(src, dst).is_ok()) {
                return false;
            }
            let mut out = vec![0u8; pixels.len()];
            if transform.transform(pixels, &mut out).is_err() {
                return false;
            }
            *pixels = out;
        }
        Samples::U16(pixels) => {
            let Ok(transform) =
                source.create_transform_16bit(Layout::Rgba, &srgb, Layout::Rgba, options)
            else {
                return false;
            };
            let mut out = vec![0u16; pixels.len()];
            if transform.transform(pixels, &mut out).is_err() {
                return false;
            }
            *pixels = out;
        }
    }
    true
}

/// Passa algumas cores de teste (preto, branco, primárias, cinza) pelo perfil.
/// Se nenhuma muda, o perfil é, na prática, o próprio sRGB.
fn is_equivalent_to_srgb(run: impl Fn(&[u8], &mut [u8]) -> bool) -> bool {
    const PROBE: [u8; 28] = [
        0, 0, 0, 255, //
        255, 255, 255, 255, //
        255, 0, 0, 255, //
        0, 255, 0, 255, //
        0, 0, 255, 255, //
        128, 128, 128, 255, //
        200, 90, 30, 255,
    ];
    let mut out = [0u8; 28];
    if !run(&PROBE, &mut out) {
        return false;
    }
    PROBE
        .iter()
        .zip(&out)
        .all(|(a, b)| (i32::from(*a) - i32::from(*b)).abs() <= SAME_AS_SRGB_TOLERANCE)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn p3() -> Vec<u8> {
        ColorProfile::new_display_p3()
            .encode()
            .expect("perfil P3 gera ICC")
    }

    #[test]
    fn srgb_nao_muda_nada() {
        let icc = ColorProfile::new_srgb().encode().unwrap();
        let mut r = Raster::from_samples(1, 1, Samples::U8(vec![200, 90, 30, 255]));
        assert!(!to_srgb(&mut r, &icc));
        let Samples::U8(v) = &r.samples else { panic!() };
        assert_eq!(v, &[200, 90, 30, 255]);
    }

    #[test]
    fn display_p3_e_convertido_e_o_alfa_sobrevive() {
        // O verde puro do P3 fica fora do sRGB: ao converter, satura em 255
        // no verde e perde o vermelho e o azul que sobrariam.
        let mut r = Raster::from_samples(1, 1, Samples::U8(vec![0, 255, 0, 77]));
        assert!(to_srgb(&mut r, &p3()));
        let Samples::U8(v) = &r.samples else { panic!() };
        assert_eq!(v[3], 77, "alfa intacto");
        assert!(v[1] >= 250 && v[0] <= 5, "verde do P3 → sRGB: {v:?}");
    }

    #[test]
    fn cinza_do_p3_muda_pouco_e_16_bits_funciona() {
        let mut r = Raster::from_samples(1, 1, Samples::U16(vec![0, 65_535, 0, 65_535]));
        assert!(to_srgb(&mut r, &p3()));
        let Samples::U16(v) = &r.samples else {
            panic!()
        };
        assert!(v[1] > 60_000);
    }

    #[test]
    fn perfil_ilegivel_e_ignorado() {
        let mut r = Raster::from_samples(1, 1, Samples::U8(vec![1, 2, 3, 255]));
        assert!(!to_srgb(&mut r, b"isto nao e um perfil icc"));
        assert!(!to_srgb(&mut r, &[]));
    }
}
