#!/usr/bin/env sh
# Compila a versão web do Morsa (Linux, macOS ou Git Bash no Windows).
#   1. núcleo Rust → WASM em app/web/pkg (exige Rust nightly + rust-src, ver README)
#   2. app Flutter → app/build/web, sem recursos de CDN (RNF10)
set -eu
cd "$(dirname "$0")/../app"
dart run flutter_rust_bridge build-web --dart-root . --rust-root ../core --output "$(pwd)/web" --release
flutter build web --release --no-web-resources-cdn --no-wasm-dry-run
echo "Pronto. Para abrir: python scripts/serve_web.py  →  http://localhost:8080"
