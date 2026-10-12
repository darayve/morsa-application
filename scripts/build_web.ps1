# Compila a versão web do Morsa (Windows, PowerShell).
#   1. núcleo Rust → WASM em app/web/pkg (exige Rust nightly + rust-src, ver README)
#   2. app Flutter → app/build/web, sem recursos de CDN (RNF10)
$ErrorActionPreference = 'Stop'
Push-Location (Join-Path $PSScriptRoot '..\app')
try {
    dart run flutter_rust_bridge build-web --dart-root . --rust-root ..\core --output (Join-Path (Get-Location) 'web') --release
    if ($LASTEXITCODE -ne 0) { throw 'Falha ao compilar o núcleo para WASM' }
    flutter build web --release --no-web-resources-cdn --no-wasm-dry-run
    if ($LASTEXITCODE -ne 0) { throw 'Falha ao compilar o app web' }
    Write-Host 'Pronto. Para abrir: python scripts/serve_web.py  ->  http://localhost:8080'
} finally {
    Pop-Location
}
