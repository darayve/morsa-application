//! Gera os arquivos de teste de `corpus/image/` (TST02, TST09).
//!
//! Rodar a partir de `core/`:  `cargo run --example gen_corpus`
//!
//! A geração é determinística: rodar de novo produz os mesmos bytes, então os
//! arquivos versionados podem ser conferidos a qualquer momento.

use std::fs;
use std::io::Cursor;
use std::path::Path;

use ::image::codecs::jpeg::JpegEncoder;
use ::image::{ImageBuffer, ImageFormat, Rgb, Rgba, RgbaImage};

fn main() {
    let png_dir = Path::new("../corpus/image/png");
    let jpg_dir = Path::new("../corpus/image/jpg");
    fs::create_dir_all(png_dir).unwrap();
    fs::create_dir_all(jpg_dir).unwrap();

    // 64 × 48: metade esquerda opaca com degradê, metade direita totalmente
    // transparente e uma coluna central com 50% de alfa.
    let rgba: RgbaImage = ImageBuffer::from_fn(64, 48, |x, y| {
        let alpha = match x {
            0..=31 => 255,
            32 => 128,
            _ => 0,
        };
        Rgba([(x * 4) as u8, (y * 5) as u8, 200, alpha])
    });
    let valid_png = encode(&::image::DynamicImage::ImageRgba8(rgba), ImageFormat::Png);
    write(png_dir, "valido-rgba.png", &valid_png);

    // 16 bits por canal (§5.3: reduzir para 8 bits com arredondamento).
    let rgba16: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::from_fn(32, 32, |x, y| {
        Rgba([(x * 2000) as u16, (y * 2000) as u16, 40_000, 65_535])
    });
    write(
        png_dir,
        "valido-16bit.png",
        &encode(
            &::image::DynamicImage::ImageRgba16(rgba16),
            ImageFormat::Png,
        ),
    );

    // Truncado: o cabeçalho está inteiro, mas o arquivo acaba no meio dos dados.
    write(
        png_dir,
        "truncado.png",
        &valid_png[..valid_png.len() * 6 / 10],
    );

    // Assinatura PNG correta seguida de lixo.
    let mut bad_header = valid_png[..8].to_vec();
    bad_header.extend_from_slice(b"isto nao e um chunk IHDR valido.");
    write(png_dir, "cabecalho-invalido.png", &bad_header);

    // Texto com extensão .png: nenhuma assinatura conhecida.
    write(
        png_dir,
        "nao-e-png.png",
        b"Ola! Eu sou um arquivo de texto.\n",
    );

    // 0 bytes.
    write(png_dir, "vazio.png", b"");

    // PNG de verdade com extensão .jpg (RN14).
    write(png_dir, "png-extensao-errada.jpg", &valid_png);

    // "Decompression bomb": cabeçalho declara 40 000 × 40 000 (1,6 GP).
    write(
        png_dir,
        "dimensoes-gigantes.png",
        &png_header_only(40_000, 40_000),
    );

    // JPG simples, para testar a regra do mesmo formato (E-101).
    let rgb = ImageBuffer::from_fn(40, 30, |x, y| Rgb([(x * 6) as u8, (y * 8) as u8, 120]));
    let mut jpg = Vec::new();
    JpegEncoder::new_with_quality(&mut jpg, 90)
        .encode_image(&rgb)
        .unwrap();
    write(jpg_dir, "valido.jpg", &jpg);

    println!("corpus gerado");
}

fn encode(image: &::image::DynamicImage, format: ImageFormat) -> Vec<u8> {
    let mut out = Cursor::new(Vec::new());
    image.write_to(&mut out, format).unwrap();
    out.into_inner()
}

fn write(dir: &Path, name: &str, bytes: &[u8]) {
    fs::write(dir.join(name), bytes).unwrap();
    println!("{:>8} B  {}", bytes.len(), dir.join(name).display());
}

/// Monta um PNG pequeno que declara dimensões enormes: assinatura, IHDR,
/// um IDAT minúsculo (zlib vazio) e IEND, todos com CRC correto.
fn png_header_only(width: u32, height: u32) -> Vec<u8> {
    let mut ihdr = Vec::new();
    ihdr.extend_from_slice(&width.to_be_bytes());
    ihdr.extend_from_slice(&height.to_be_bytes());
    ihdr.extend_from_slice(&[8, 6, 0, 0, 0]); // 8 bits, RGBA, sem entrelaçamento

    let mut out = vec![0x89, b'P', b'N', b'G', 0x0D, 0x0A, 0x1A, 0x0A];
    push_chunk(&mut out, b"IHDR", &ihdr);
    push_chunk(
        &mut out,
        b"IDAT",
        &[0x78, 0x9C, 0x03, 0x00, 0x00, 0x00, 0x00, 0x01],
    );
    push_chunk(&mut out, b"IEND", &[]);
    out
}

/// Acrescenta um chunk PNG: tamanho, tipo, dados e CRC do tipo + dados.
fn push_chunk(out: &mut Vec<u8>, kind: &[u8; 4], data: &[u8]) {
    let mut body = kind.to_vec();
    body.extend_from_slice(data);
    out.extend_from_slice(&(data.len() as u32).to_be_bytes());
    out.extend_from_slice(&body);
    out.extend_from_slice(&crc32(&body).to_be_bytes());
}

/// CRC-32 do PNG (polinômio 0xEDB88320), calculado bit a bit.
fn crc32(bytes: &[u8]) -> u32 {
    let mut crc = 0xFFFF_FFFFu32;
    for &byte in bytes {
        crc ^= u32::from(byte);
        for _ in 0..8 {
            let mask = (crc & 1).wrapping_neg();
            crc = (crc >> 1) ^ (0xEDB8_8320 & mask);
        }
    }
    !crc
}
