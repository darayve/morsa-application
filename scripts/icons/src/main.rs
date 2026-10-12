//! Gera os ícones de cada plataforma a partir de `assets/icon.svg` (§9.10).
//!
//! Rodar a partir da raiz do repositório:
//!     cargo run --release --manifest-path scripts/icons/Cargo.toml
//!
//! É uma ferramenta de desenvolvimento: roda só quando o ícone muda, e os
//! arquivos gerados ficam versionados. Nada disso entra no app em execução.

use std::fs;
use std::path::{Path, PathBuf};

use image::codecs::ico::{IcoEncoder, IcoFrame};
use image::{ExtendedColorType, ImageFormat, RgbaImage};
use resvg::{tiny_skia, usvg};

/// Cor de fundo do próprio ícone, usada na margem da versão "maskable".
const BACKGROUND: [u8; 4] = [0xe9, 0xe9, 0xe5, 0xff];

fn main() {
    let root = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..");
    let svg = fs::read(root.join("assets/icon.svg")).expect("assets/icon.svg não encontrado");
    // `usvg` lê o SVG; o padrão não busca nada externo, e o nosso ícone não tem referências.
    let tree = usvg::Tree::from_data(&svg, &usvg::Options::default()).expect("SVG inválido");
    let app = root.join("app");

    // Windows: um .ico com vários tamanhos.
    let ico_sizes = [16, 24, 32, 48, 64, 128, 256];
    let frames: Vec<IcoFrame> = ico_sizes
        .iter()
        .map(|&s| {
            let png = png_bytes(&render(&tree, s, 1.0));
            IcoFrame::with_encoded(png, s, s, ExtendedColorType::Rgba8).unwrap()
        })
        .collect();
    let ico_path = app.join("windows/runner/resources/app_icon.ico");
    IcoEncoder::new(fs::File::create(&ico_path).unwrap())
        .encode_images(&frames)
        .unwrap();
    println!("{}", ico_path.display());

    // macOS: PNGs do catálogo AppIcon (o Xcode monta o .icns na compilação).
    for s in [16, 32, 64, 128, 256, 512, 1024] {
        save(
            &render(&tree, s, 1.0),
            &app.join(format!(
                "macos/Runner/Assets.xcassets/AppIcon.appiconset/app_icon_{s}.png"
            )),
        );
    }

    // Linux: tamanhos do tema hicolor, usados no empacotamento (Marco 6).
    for s in [16, 24, 32, 48, 64, 128, 256, 512] {
        save(
            &render(&tree, s, 1.0),
            &app.join(format!("linux/icons/hicolor/{s}x{s}/apps/morsa.png")),
        );
    }

    // Web/PWA: favicon, ícones normais e "maskable" com 10% de margem de segurança.
    save(&render(&tree, 32, 1.0), &app.join("web/favicon.png"));
    for s in [192, 512] {
        save(
            &render(&tree, s, 1.0),
            &app.join(format!("web/icons/Icon-{s}.png")),
        );
        save(
            &render(&tree, s, 0.8),
            &app.join(format!("web/icons/Icon-maskable-{s}.png")),
        );
    }
}

/// Renderiza o ícone num quadrado de `size` px. `fill` < 1 encolhe o desenho
/// e completa a borda com a cor de fundo (ícones "maskable").
fn render(tree: &usvg::Tree, size: u32, fill: f32) -> RgbaImage {
    let mut pixmap = tiny_skia::Pixmap::new(size, size).unwrap();
    if fill < 1.0 {
        let [r, g, b, a] = BACKGROUND;
        pixmap.fill(tiny_skia::Color::from_rgba8(r, g, b, a));
    }
    let drawn = size as f32 * fill;
    let scale = drawn / tree.size().width();
    let offset = (size as f32 - drawn) / 2.0;
    let transform = tiny_skia::Transform::from_scale(scale, scale).post_translate(offset, offset);
    resvg::render(tree, transform, &mut pixmap.as_mut());

    // O tiny-skia guarda cores pré-multiplicadas pelo alfa; o PNG quer cores normais.
    let mut out = RgbaImage::new(size, size);
    for (dst, src) in out.pixels_mut().zip(pixmap.pixels()) {
        let c = src.demultiply();
        dst.0 = [c.red(), c.green(), c.blue(), c.alpha()];
    }
    out
}

fn png_bytes(image: &RgbaImage) -> Vec<u8> {
    let mut out = std::io::Cursor::new(Vec::new());
    image.write_to(&mut out, ImageFormat::Png).unwrap();
    out.into_inner()
}

fn save(image: &RgbaImage, path: &Path) {
    fs::create_dir_all(path.parent().unwrap()).unwrap();
    image.save_with_format(path, ImageFormat::Png).unwrap();
    println!("{}", path.display());
}
