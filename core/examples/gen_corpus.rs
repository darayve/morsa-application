//! Gera os arquivos de teste de `corpus/image/` (TST02, TST09).
//!
//! Rodar a partir de `core/`:  `cargo run --example gen_corpus`
//!
//! A geração é determinística: rodar de novo produz os mesmos bytes, então os
//! arquivos versionados podem ser conferidos a qualquer momento (a CI faz isso).
//!
//! Os arquivos válidos são gravados por codificadores independentes do núcleo
//! (`image`, `gif`, `tiff`, `jpeg-encoder`), para que ler com o núcleo seja um
//! teste de verdade, e não o núcleo lendo o que ele mesmo escreveu.

use std::fs;
use std::io::Cursor;
use std::path::Path;

use ::image::codecs::gif::GifEncoder;
use ::image::codecs::ico::{IcoEncoder, IcoFrame};
use ::image::codecs::jpeg::JpegEncoder;
use ::image::codecs::png::PngEncoder;
use ::image::{
    DynamicImage, ExtendedColorType, Frame, ImageBuffer, ImageEncoder, ImageFormat, Luma, LumaA,
    Rgb, Rgba, RgbaImage,
};

const ROOT: &str = "../corpus/image";

fn main() {
    png();
    jpg();
    webp();
    gif();
    bmp();
    tiff();
    ico();
    svg();
    println!("corpus gerado");
}

// ---------------------------------------------------------------------------
// PNG
// ---------------------------------------------------------------------------

fn png() {
    let dir = dir("png");

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
    let valid_png = encode(&DynamicImage::ImageRgba8(rgba), ImageFormat::Png);
    write(&dir, "valido-rgba.png", &valid_png);

    // 16 bits por canal (§5.3: reduzir para 8 bits com arredondamento).
    let rgba16: ImageBuffer<Rgba<u16>, Vec<u16>> = ImageBuffer::from_fn(32, 32, |x, y| {
        Rgba([(x * 2000) as u16, (y * 2000) as u16, 40_000, 65_535])
    });
    write(
        &dir,
        "valido-16bit.png",
        &encode(&DynamicImage::ImageRgba16(rgba16), ImageFormat::Png),
    );

    // Truncado: o cabeçalho está inteiro, mas o arquivo acaba no meio dos dados.
    write(&dir, "truncado.png", &valid_png[..valid_png.len() * 6 / 10]);

    // Assinatura PNG correta seguida de lixo.
    let mut bad_header = valid_png[..8].to_vec();
    bad_header.extend_from_slice(b"isto nao e um chunk IHDR valido.");
    write(&dir, "cabecalho-invalido.png", &bad_header);

    // Texto com extensão .png: nenhuma assinatura conhecida.
    write(&dir, "nao-e-png.png", b"Ola! Eu sou um arquivo de texto.\n");

    // 0 bytes.
    write(&dir, "vazio.png", b"");

    // PNG de verdade com extensão .jpg (RN14).
    write(&dir, "png-extensao-errada.jpg", &valid_png);

    // "Decompression bomb": cabeçalho declara 40 000 × 40 000 (1,6 GP).
    write(
        &dir,
        "dimensoes-gigantes.png",
        &png_header_only(40_000, 40_000),
    );

    // Arquivo válido mínimo: 1 × 1.
    write(
        &dir,
        "minimo.png",
        &encode(
            &DynamicImage::ImageRgba8(ImageBuffer::from_pixel(1, 1, Rgba([10, 20, 30, 255]))),
            ImageFormat::Png,
        ),
    );

    // Típico: 64 × 48 opaco.
    write(
        &dir,
        "tipico.png",
        &encode(&typical_dyn(64, 48), ImageFormat::Png),
    );

    // Cinza (8 bits) e cinza com alfa.
    let gray: ImageBuffer<Luma<u8>, Vec<u8>> =
        ImageBuffer::from_fn(32, 32, |x, y| Luma([((x + y) * 4) as u8]));
    write(
        &dir,
        "cinza.png",
        &encode(&DynamicImage::ImageLuma8(gray), ImageFormat::Png),
    );
    let gray_alpha: ImageBuffer<LumaA<u8>, Vec<u8>> = ImageBuffer::from_fn(32, 32, |x, y| {
        LumaA([(x * 8) as u8, if y < 16 { 255 } else { 90 }])
    });
    write(
        &dir,
        "cinza-alfa.png",
        &encode(&DynamicImage::ImageLumaA8(gray_alpha), ImageFormat::Png),
    );

    // Perfil de cor Display P3 (o verde puro do P3 fica fora do sRGB).
    let p3 = ImageBuffer::from_fn(32, 32, |_, _| Rgb([200u8, 100, 50]));
    let mut with_icc = Vec::new();
    let mut encoder = PngEncoder::new(&mut with_icc);
    encoder.set_icc_profile(p3_profile()).unwrap();
    encoder
        .write_image(p3.as_raw(), 32, 32, ExtendedColorType::Rgb8)
        .unwrap();
    write(&dir, "icc-p3.png", &with_icc);

    // Grande: 3000 × 2000 (6 MP), que comprime bem por ser um degradê.
    write(
        &dir,
        "grande.png",
        &encode(&typical_dyn(3000, 2000), ImageFormat::Png),
    );
}

// ---------------------------------------------------------------------------
// JPG
// ---------------------------------------------------------------------------

fn jpg() {
    let dir = dir("jpg");

    // JPG simples, para testar a regra do mesmo formato (E-101).
    let rgb = ImageBuffer::from_fn(40, 30, |x, y| Rgb([(x * 6) as u8, (y * 8) as u8, 120]));
    let mut jpg = Vec::new();
    JpegEncoder::new_with_quality(&mut jpg, 90)
        .encode_image(&rgb)
        .unwrap();
    write(&dir, "valido.jpg", &jpg);

    let typical_rgb = typical_dyn(64, 48).to_rgb8();
    let tipico = jpeg(typical_rgb.as_raw(), 64, 48, ExtendedColorType::Rgb8, 90);
    write(&dir, "tipico.jpg", &tipico);
    write(
        &dir,
        "minimo.jpg",
        &jpeg(&[200, 100, 50], 1, 1, ExtendedColorType::Rgb8, 90),
    );
    write(&dir, "truncado.jpg", &tipico[..tipico.len() * 6 / 10]);

    let mut bad = vec![0xFF, 0xD8, 0xFF];
    bad.extend_from_slice(b"isto nao e um segmento JPEG valido, so lixo.");
    write(&dir, "cabecalho-invalido.jpg", &bad);

    // JPG de verdade com extensão .png (RN14).
    write(&dir, "jpg-extensao-errada.png", &tipico);

    // Cinza (1 canal).
    let gray: Vec<u8> = (0..32 * 32).map(|i| ((i % 32) * 8) as u8).collect();
    write(
        &dir,
        "cinza.jpg",
        &jpeg(&gray, 32, 32, ExtendedColorType::L8, 90),
    );

    // CMYK: quatro faixas verticais de cor pura (ciano, magenta, amarelo, preto).
    // O formato guarda CMYK invertido por convenção do Adobe; `jpeg-encoder`
    // escreve do jeito que os programas esperam ler.
    let mut cmyk = Vec::new();
    for _y in 0..32 {
        for x in 0..32 {
            cmyk.extend_from_slice(match x / 8 {
                0 => &[255, 0, 0, 0],
                1 => &[0, 255, 0, 0],
                2 => &[0, 0, 255, 0],
                _ => &[0, 0, 0, 255],
            });
        }
    }
    let mut cmyk_jpg = Vec::new();
    jpeg_encoder::Encoder::new(&mut cmyk_jpg, 95)
        .encode(&cmyk, 32, 32, jpeg_encoder::ColorType::Cmyk)
        .unwrap();
    write(&dir, "cmyk.jpg", &cmyk_jpg);

    // Perfil Display P3.
    let green = ImageBuffer::from_fn(32, 32, |_, _| Rgb([200u8, 100, 50]));
    let mut with_icc = Vec::new();
    let mut encoder = JpegEncoder::new_with_quality(&mut with_icc, 95);
    encoder.set_icc_profile(p3_profile()).unwrap();
    encoder
        .write_image(green.as_raw(), 32, 32, ExtendedColorType::Rgb8)
        .unwrap();
    write(&dir, "icc-p3.jpg", &with_icc);

    // Orientação EXIF 1 a 8. A imagem "como deve ser vista" está em
    // `exif-canonico.png`; cada `exif-N.jpg` guarda os pixels deitados,
    // espelhados ou girados, mais a etiqueta que manda corrigir (RF07, §5.3).
    let canonical = orientation_canonical();
    write(
        &dir,
        "exif-canonico.png",
        &encode(
            &DynamicImage::ImageRgba8(canonical.clone()),
            ImageFormat::Png,
        ),
    );
    for n in 1..=8u8 {
        let stored = DynamicImage::ImageRgba8(to_stored(&canonical, n)).to_rgb8();
        let plain = jpeg(
            stored.as_raw(),
            stored.width(),
            stored.height(),
            ExtendedColorType::Rgb8,
            98,
        );
        write(
            &dir,
            &format!("exif-{n}.jpg"),
            &with_exif_orientation(&plain, n),
        );
    }

    // Grande: 3000 × 2000.
    let big = typical_dyn(3000, 2000).to_rgb8();
    write(
        &dir,
        "grande.jpg",
        &jpeg(big.as_raw(), 3000, 2000, ExtendedColorType::Rgb8, 85),
    );
}

fn jpeg(data: &[u8], w: u32, h: u32, color: ExtendedColorType, quality: u8) -> Vec<u8> {
    let mut out = Vec::new();
    JpegEncoder::new_with_quality(&mut out, quality)
        .write_image(data, w, h, color)
        .unwrap();
    out
}

/// Imagem 40 × 30 com 4 quadrantes de cores diferentes e um quadrado branco
/// perto do canto superior esquerdo: qualquer espelhamento ou giro muda onde
/// cada cor aparece.
pub fn orientation_canonical() -> RgbaImage {
    ImageBuffer::from_fn(40, 30, |x, y| {
        let base = match (x < 20, y < 15) {
            (true, true) => [220, 30, 30],    // topo esquerdo: vermelho
            (false, true) => [30, 200, 30],   // topo direito: verde
            (true, false) => [30, 30, 220],   // base esquerda: azul
            (false, false) => [230, 210, 30], // base direita: amarelo
        };
        if (2..8).contains(&x) && (2..8).contains(&y) {
            Rgba([255, 255, 255, 255])
        } else {
            Rgba([base[0], base[1], base[2], 255])
        }
    })
}

/// Para onde vai o pixel (x, y) de uma imagem guardada W × H quando a etiqueta
/// EXIF é `n`, na imagem exibida (especificação EXIF, tabela de orientações).
fn display_position(n: u8, x: u32, y: u32, w: u32, h: u32) -> (u32, u32) {
    match n {
        1 => (x, y),
        2 => (w - 1 - x, y),
        3 => (w - 1 - x, h - 1 - y),
        4 => (x, h - 1 - y),
        5 => (y, x),
        6 => (h - 1 - y, x),
        7 => (h - 1 - y, w - 1 - x),
        8 => (y, w - 1 - x),
        _ => unreachable!(),
    }
}

/// Os pixels que precisam ser guardados para que a orientação `n` mostre
/// `canonical`. Em 5 a 8 a imagem guardada é "em pé" (30 × 40).
fn to_stored(canonical: &RgbaImage, n: u8) -> RgbaImage {
    let (cw, ch) = canonical.dimensions();
    let (sw, sh) = if n >= 5 { (ch, cw) } else { (cw, ch) };
    ImageBuffer::from_fn(sw, sh, |x, y| {
        let (dx, dy) = display_position(n, x, y, sw, sh);
        *canonical.get_pixel(dx, dy)
    })
}

/// Insere um segmento EXIF (APP1) com só a etiqueta de orientação logo depois
/// do SOI do JPEG.
fn with_exif_orientation(jpeg: &[u8], orientation: u8) -> Vec<u8> {
    let mut exif = Vec::new();
    exif.extend_from_slice(b"Exif\0\0");
    exif.extend_from_slice(b"MM\0\x2A\0\0\0\x08"); // TIFF big endian, IFD em 8
    exif.extend_from_slice(&1u16.to_be_bytes()); // 1 entrada
    exif.extend_from_slice(&0x0112u16.to_be_bytes()); // Orientation
    exif.extend_from_slice(&3u16.to_be_bytes()); // tipo SHORT
    exif.extend_from_slice(&1u32.to_be_bytes()); // 1 valor
    exif.extend_from_slice(&[0, orientation, 0, 0]); // valor, alinhado à esquerda
    exif.extend_from_slice(&0u32.to_be_bytes()); // sem próximo IFD

    let mut out = vec![0xFF, 0xD8, 0xFF, 0xE1];
    out.extend_from_slice(&((exif.len() + 2) as u16).to_be_bytes());
    out.extend_from_slice(&exif);
    out.extend_from_slice(&jpeg[2..]);
    out
}

/// Perfil ICC do Display P3, gerado pelo `moxcms` (mesmo crate que o núcleo
/// usa para converter, mas aqui só serve para fabricar o arquivo de teste).
fn p3_profile() -> Vec<u8> {
    moxcms::ColorProfile::new_display_p3().encode().unwrap()
}

// ---------------------------------------------------------------------------
// WebP
// ---------------------------------------------------------------------------

fn webp() {
    let dir = dir("webp");
    let lossless = |img: &RgbaImage| {
        let mut out = Vec::new();
        ::image::codecs::webp::WebPEncoder::new_lossless(&mut out)
            .write_image(
                img.as_raw(),
                img.width(),
                img.height(),
                ExtendedColorType::Rgba8,
            )
            .unwrap();
        out
    };

    let typical = typical(64, 48);
    let tipico = lossless(&typical);
    write(&dir, "tipico.webp", &tipico);
    write(
        &dir,
        "minimo.webp",
        &lossless(&ImageBuffer::from_pixel(1, 1, Rgba([9, 99, 199, 255]))),
    );
    write(&dir, "truncado.webp", &tipico[..tipico.len() * 6 / 10]);

    let mut bad = b"RIFF\x20\0\0\0WEBPVP8 ".to_vec();
    bad.extend_from_slice(b"isto nao e um fluxo VP8 valido.");
    write(&dir, "cabecalho-invalido.webp", &bad);

    // Transparente (sem perdas): metade direita com alfa.
    let transparent: RgbaImage = ImageBuffer::from_fn(64, 48, |x, y| {
        Rgba([
            (x * 4) as u8,
            (y * 5) as u8,
            200,
            if x < 32 { 255 } else { 100 },
        ])
    });
    write(&dir, "transparente.webp", &lossless(&transparent));

    // Com perdas (VP8).
    let rgb = DynamicImage::ImageRgba8(typical.clone()).to_rgb8();
    let lossy = oxideav_webp::encode_rgb8(
        64,
        48,
        rgb.as_raw(),
        &oxideav_webp::EncodeOptions::default().with_quality(85.0),
    )
    .unwrap();
    write(&dir, "com-perdas.webp", &lossy);

    // Animado: dois quadros (vermelho e azul) num contêiner VP8X + ANIM + ANMF.
    let red = lossless(&ImageBuffer::from_pixel(16, 16, Rgba([255, 0, 0, 255])));
    let blue = lossless(&ImageBuffer::from_pixel(16, 16, Rgba([0, 0, 255, 255])));
    write(&dir, "animado.webp", &animated_webp(&[&red, &blue], 16, 16));
}

/// Monta um WebP animado com os quadros (arquivos WebP sem perdas simples).
fn animated_webp(frames: &[&[u8]], w: u32, h: u32) -> Vec<u8> {
    let mut body = b"WEBP".to_vec();
    let mut vp8x = vec![0x02, 0, 0, 0]; // flag de animação
    vp8x.extend_from_slice(&(w - 1).to_le_bytes()[..3]);
    vp8x.extend_from_slice(&(h - 1).to_le_bytes()[..3]);
    riff_chunk(&mut body, b"VP8X", &vp8x);
    riff_chunk(&mut body, b"ANIM", &[0, 0, 0, 0, 0, 0]); // fundo e repetições
    for frame in frames {
        let payload = &frame[12..]; // chunks do quadro (VP8L ...), sem o RIFF
        let mut anmf = Vec::new();
        anmf.extend_from_slice(&[0, 0, 0, 0, 0, 0]); // x, y
        anmf.extend_from_slice(&(w - 1).to_le_bytes()[..3]);
        anmf.extend_from_slice(&(h - 1).to_le_bytes()[..3]);
        anmf.extend_from_slice(&[100, 0, 0]); // 100 ms
        anmf.push(0); // sem mistura especial
        anmf.extend_from_slice(payload);
        riff_chunk(&mut body, b"ANMF", &anmf);
    }
    let mut out = b"RIFF".to_vec();
    out.extend_from_slice(&(body.len() as u32).to_le_bytes());
    out.extend_from_slice(&body);
    out
}

fn riff_chunk(out: &mut Vec<u8>, name: &[u8; 4], data: &[u8]) {
    out.extend_from_slice(name);
    out.extend_from_slice(&(data.len() as u32).to_le_bytes());
    out.extend_from_slice(data);
    if data.len() % 2 == 1 {
        out.push(0);
    }
}

// ---------------------------------------------------------------------------
// GIF
// ---------------------------------------------------------------------------

fn gif() {
    let dir = dir("gif");
    let frame = |img: RgbaImage| Frame::new(img);
    let encode_gif = |frames: Vec<Frame>| {
        let mut out = Vec::new();
        {
            let mut encoder = GifEncoder::new(&mut out);
            encoder.encode_frames(frames).unwrap();
        }
        out
    };

    let tipico = encode_gif(vec![frame(typical(64, 48))]);
    write(&dir, "tipico.gif", &tipico);
    write(
        &dir,
        "minimo.gif",
        &encode_gif(vec![frame(ImageBuffer::from_pixel(
            1,
            1,
            Rgba([50, 150, 250, 255]),
        ))]),
    );
    write(&dir, "truncado.gif", &tipico[..tipico.len() * 6 / 10]);

    let mut bad = b"GIF89a".to_vec();
    bad.extend_from_slice(b"isto nao e uma tela GIF valida.");
    write(&dir, "cabecalho-invalido.gif", &bad);

    let transparent: RgbaImage = ImageBuffer::from_fn(32, 32, |x, _| {
        Rgba([200, 40, 40, if x < 16 { 255 } else { 0 }])
    });
    write(
        &dir,
        "transparente.gif",
        &encode_gif(vec![frame(transparent)]),
    );

    // Animado: 3 quadros de cores diferentes.
    let solid = |c: [u8; 3]| {
        frame(ImageBuffer::from_pixel(
            16,
            16,
            Rgba([c[0], c[1], c[2], 255]),
        ))
    };
    write(
        &dir,
        "animado.gif",
        &encode_gif(vec![
            solid([255, 0, 0]),
            solid([0, 255, 0]),
            solid([0, 0, 255]),
        ]),
    );
}

// ---------------------------------------------------------------------------
// BMP
// ---------------------------------------------------------------------------

fn bmp() {
    let dir = dir("bmp");
    let rgb = typical_dyn(64, 48).to_rgb8();
    let tipico = encode(&DynamicImage::ImageRgb8(rgb), ImageFormat::Bmp);
    write(&dir, "tipico.bmp", &tipico);
    write(
        &dir,
        "minimo.bmp",
        &encode(
            &DynamicImage::ImageRgb8(ImageBuffer::from_pixel(1, 1, Rgb([1, 2, 3]))),
            ImageFormat::Bmp,
        ),
    );
    write(&dir, "truncado.bmp", &tipico[..tipico.len() * 6 / 10]);

    // Assinatura e tamanho do cabeçalho corretos, mas largura e altura zeradas.
    let mut bad = tipico.clone();
    bad[18..26].fill(0);
    write(&dir, "cabecalho-invalido.bmp", &bad);
}

// ---------------------------------------------------------------------------
// TIFF
// ---------------------------------------------------------------------------

fn tiff() {
    use ::tiff::encoder::{Compression, TiffEncoder, colortype};
    use ::tiff::tags::ExtraSamples;

    let dir = dir("tiff");
    let rgb = typical_dyn(64, 48).to_rgb8();

    let tipico = {
        let mut out = Cursor::new(Vec::new());
        TiffEncoder::new(&mut out)
            .unwrap()
            .with_compression(Compression::Lzw)
            .write_image::<colortype::RGB8>(64, 48, rgb.as_raw())
            .unwrap();
        out.into_inner()
    };
    write(&dir, "tipico.tiff", &tipico);
    write(&dir, "truncado.tiff", &tipico[..tipico.len() * 6 / 10]);

    let mut minimo = Cursor::new(Vec::new());
    TiffEncoder::new(&mut minimo)
        .unwrap()
        .write_image::<colortype::RGB8>(1, 1, &[7, 8, 9])
        .unwrap();
    write(&dir, "minimo.tiff", &minimo.into_inner());

    let mut bad = vec![0x49, 0x49, 0x2A, 0x00];
    bad.extend_from_slice(b"isto nao e um diretorio TIFF valido.");
    write(&dir, "cabecalho-invalido.tiff", &bad);

    // RGBA com alfa "não associado".
    let rgba = typical(32, 32);
    let mut transparent = Cursor::new(Vec::new());
    {
        let mut enc = TiffEncoder::new(&mut transparent).unwrap();
        let mut image = enc.new_image::<colortype::RGB8>(32, 32).unwrap();
        image
            .extra_samples(&[ExtraSamples::UnassociatedAlpha])
            .unwrap();
        let data: Vec<u8> = rgba
            .pixels()
            .flat_map(|p| [p[0], p[1], p[2], if p[0] < 128 { 255 } else { 80 }])
            .collect();
        image.write_data(&data).unwrap();
    }
    write(&dir, "transparente.tiff", &transparent.into_inner());

    // 16 bits por canal.
    let wide: Vec<u16> = (0..32 * 32)
        .flat_map(|i| [(i % 32 * 2000) as u16, (i / 32 * 2000) as u16, 40_000])
        .collect();
    let mut sixteen = Cursor::new(Vec::new());
    TiffEncoder::new(&mut sixteen)
        .unwrap()
        .write_image::<colortype::RGB16>(32, 32, &wide)
        .unwrap();
    write(&dir, "16bit.tiff", &sixteen.into_inner());

    // Duas páginas: a primeira vermelha, a segunda azul.
    let mut pages = Cursor::new(Vec::new());
    {
        let mut enc = TiffEncoder::new(&mut pages).unwrap();
        enc.write_image::<colortype::RGB8>(8, 8, &[255, 0, 0].repeat(64))
            .unwrap();
        enc.write_image::<colortype::RGB8>(8, 8, &[0, 0, 255].repeat(64))
            .unwrap();
    }
    write(&dir, "multipagina.tiff", &pages.into_inner());
}

// ---------------------------------------------------------------------------
// ICO
// ---------------------------------------------------------------------------

fn ico() {
    let dir = dir("ico");

    // Três tamanhos, cada um de uma cor, para ver qual quadro foi usado.
    let frame = |size: u32, c: [u8; 4]| {
        encode(
            &DynamicImage::ImageRgba8(ImageBuffer::from_pixel(size, size, Rgba(c))),
            ImageFormat::Png,
        )
    };
    let build = |entries: &[(u32, [u8; 4])]| {
        let pngs: Vec<(u32, Vec<u8>)> = entries.iter().map(|(s, c)| (*s, frame(*s, *c))).collect();
        let frames: Vec<IcoFrame> = pngs
            .iter()
            .map(|(s, png)| {
                IcoFrame::with_encoded(png.clone(), *s, *s, ExtendedColorType::Rgba8).unwrap()
            })
            .collect();
        let mut out = Vec::new();
        IcoEncoder::new(&mut out).encode_images(&frames).unwrap();
        out
    };

    let multi = build(&[
        (16, [255, 0, 0, 255]),
        (32, [0, 255, 0, 255]),
        (48, [0, 0, 255, 255]),
    ]);
    write(&dir, "multitamanho.ico", &multi);
    write(&dir, "minimo.ico", &build(&[(16, [10, 20, 30, 255])]));
    write(&dir, "truncado.ico", &multi[..multi.len() * 6 / 10]);

    let mut bad = vec![0, 0, 1, 0, 1, 0];
    bad.extend_from_slice(b"isto nao e uma entrada de diretorio ICO valida.");
    write(&dir, "cabecalho-invalido.ico", &bad);
}

// ---------------------------------------------------------------------------
// SVG
// ---------------------------------------------------------------------------

fn svg() {
    let dir = dir("svg");
    let simple = r##"<svg xmlns="http://www.w3.org/2000/svg" width="64" height="48">
  <rect width="32" height="48" fill="#dc1e1e"/>
  <circle cx="48" cy="24" r="12" fill="#1e78dc"/>
</svg>
"##;
    write(&dir, "simples.svg", simple.as_bytes());
    write(
        &dir,
        "minimo.svg",
        br#"<svg xmlns="http://www.w3.org/2000/svg" width="1" height="1"/>"#,
    );
    write(
        &dir,
        "texto.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" width="96" height="40">
  <text x="4" y="30" font-size="28" font-family="Arial" fill="#000000">Morsa</text>
</svg>
"##,
    );
    write(
        &dir,
        "so-viewbox.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 200 100">
  <rect width="200" height="100" fill="#2eaa2e"/>
</svg>
"##,
    );
    // Aponta para uma imagem na internet e para um arquivo do disco: nada disso
    // pode ser buscado (RNF10).
    write(
        &dir,
        "referencias-externas.svg",
        br##"<svg xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" width="64" height="48">
  <rect width="64" height="48" fill="#ffe14d"/>
  <image xlink:href="https://example.com/logo.png" width="32" height="48"/>
  <image xlink:href="C:\Users\alguem\segredo.png" x="32" width="32" height="48"/>
</svg>
"##,
    );
    write(
        &dir,
        "truncado.svg",
        &simple.as_bytes()[..simple.len() * 6 / 10],
    );
    // 100 000 × 100 000 px declarados: acima do limite de segurança.
    write(
        &dir,
        "dimensoes-gigantes.svg",
        br#"<svg xmlns="http://www.w3.org/2000/svg" width="100000" height="100000"/>"#,
    );
    write(
        &dir,
        "cabecalho-invalido.svg",
        b"<svg isto nao e xml valido <<<",
    );
}

// ---------------------------------------------------------------------------
// Utilitários
// ---------------------------------------------------------------------------

/// Imagem opaca com degradê e xadrez, boa para ver qualquer deformação. O
/// xadrez varia só a luminosidade (a mesma soma nos três canais): bordas de cor
/// pura limitam o PSNR de qualquer formato com perdas, por causa da
/// subamostragem de crominância 4:2:0, e atrapalhariam a medida do TST05.
fn typical(w: u32, h: u32) -> RgbaImage {
    ImageBuffer::from_fn(w, h, |x, y| {
        let checker = if (x / 8 + y / 8) % 2 == 0 { 40 } else { 0 };
        let ch = |v: u32| (v + checker).min(255) as u8;
        Rgba([ch(x * 215 / w.max(1)), ch(y * 215 / h.max(1)), ch(60), 255])
    })
}

fn typical_dyn(w: u32, h: u32) -> DynamicImage {
    DynamicImage::ImageRgba8(typical(w, h))
}

fn dir(name: &str) -> std::path::PathBuf {
    let path = Path::new(ROOT).join(name);
    fs::create_dir_all(&path).unwrap();
    path
}

fn encode(image: &DynamicImage, format: ImageFormat) -> Vec<u8> {
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
