//! Alvo de fuzzing: bytes quaisquer entram no conversor de imagens (todos os
//! formatos de entrada, inclusive SVG, e todos os destinos).
//!
//! O libFuzzer gera milhões de entradas (a partir do corpus em
//! `corpus/image`) e falha se o núcleo der panic, travar ou estourar a
//! memória. Erros tipados (`E-1xx`) são o resultado esperado para lixo.
//!
//! A função chamada é `image::convert`, e não a da ponte (`api`), porque a
//! ponte transforma panic em `E-199` e esconderia justamente o que o
//! fuzzing procura.

#![no_main]

use std::io::Cursor;

use libfuzzer_sys::fuzz_target;
use morsa_core::detect::ImageFormat;
use morsa_core::image::{ConvertOptions, SvgSize, convert};

/// Imagens válidas até 500 MP são aceitas pelo núcleo, mas decodificá-las
/// gastaria gigabytes por entrada. Aqui o fuzzing fica nas imagens pequenas;
/// os limites grandes têm testes próprios em `core/tests`.
const MAX_FUZZ_PIXELS: u64 = 4_000_000;

fuzz_target!(|data: &[u8]| {
    // O primeiro byte escolhe as opções; o resto é o arquivo.
    let Some((&choice, input)) = data.split_first() else {
        return;
    };
    if let Ok((w, h)) = ::image::ImageReader::new(Cursor::new(input))
        .with_guessed_format()
        .map_err(drop)
        .and_then(|r| r.into_dimensions().map_err(drop))
        && u64::from(w) * u64::from(h) > MAX_FUZZ_PIXELS
    {
        return;
    }
    // Os 3 bits de baixo escolhem o destino entre os 7 formatos de saída; os
    // 5 de cima viram a qualidade (inclui 0 e valores acima de 100, que
    // precisam virar E-104). O resto dos bytes é o arquivo.
    let targets = [
        ImageFormat::Png,
        ImageFormat::Jpeg,
        ImageFormat::WebP,
        ImageFormat::Gif,
        ImageFormat::Bmp,
        ImageFormat::Tiff,
        ImageFormat::Ico,
    ];
    let mut options = ConvertOptions::new(targets[usize::from(choice & 7) % targets.len()]);
    options.quality = (choice >> 3) * 4;
    options.webp_lossless = choice & 0x80 != 0;
    // SVG declara o próprio tamanho (até 16 384 px por lado = 1 GB de pixels);
    // no fuzzing, a rasterização fica pequena.
    options.svg_size = SvgSize::Pixels {
        width: Some(64),
        height: None,
    };
    let _ = convert(input, &options);
});
