//! Alvo de fuzzing: bytes quaisquer entram no conversor de imagens.
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
use morsa_core::image::{ConvertOptions, convert};

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
    let target = if choice & 1 == 0 {
        ImageFormat::Jpeg
    } else {
        ImageFormat::Png
    };
    // Inclui 0 e valores acima de 100, que precisam virar E-104.
    let quality = choice >> 1;
    let _ = convert(input, ConvertOptions { target, quality });
});
