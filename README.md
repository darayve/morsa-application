# Morsa

Utilitários para arquivos (conversão de imagens, operações com PDF e arquivos ZIP)
que rodam **100% offline** em Windows, macOS, Linux e no navegador. Sem conta, sem
servidor: nenhum arquivo sai do seu computador. Código aberto, sob
**MIT OR Apache-2.0**.

> **Situação:** em desenvolvimento (Marco 1, esqueleto). Hoje o app converte **PNG → JPG**
> de ponta a ponta no desktop e na web. O plano completo está em
> [`docs/requisitos.md`](docs/requisitos.md), §18.

## Como o projeto é organizado

```
app/        interface em Flutter (telas, tema, textos pt-BR/en)
  lib/core_api/   código gerado pela ponte (não editar)
  rust_builder/   cola do cargokit que compila o núcleo junto com o app
core/       núcleo em Rust: recebe bytes e opções, devolve bytes (sem telas, sem rede)
corpus/     arquivos de teste, válidos e corrompidos (origem em corpus/SOURCES.md)
design/     protótipos e sistema visual
docs/       requisitos, regras de negócio e decisões
scripts/    geradores (ícones, tokens de cor) e utilitários de build
```

A interface chama o núcleo pelo [flutter_rust_bridge](https://github.com/fzyzcjy/flutter_rust_bridge)
(FRB). No desktop, o núcleo é uma biblioteca nativa. Na web, é compilado para
WebAssembly e roda em Web Workers, para a interface nunca travar.

## Versões usadas

| Ferramenta | Versão |
| --- | --- |
| Flutter (stable) | 3.47.7 (Dart 3.13.5) |
| Rust stable | 1.99.0 (fixado em `rust-toolchain.toml`) |
| Rust nightly + `rust-src` | necessário só para a build web |
| flutter_rust_bridge_codegen | 2.13.0 (igual ao pacote Dart `flutter_rust_bridge`) |
| wasm-pack | 0.15.0 |
| wasm-bindgen-cli | 0.2.129 (igual à versão do `core/Cargo.lock`) |
| cargo-deny | 0.20.2 |

## Pré-requisitos

### Todos os sistemas

1. [Flutter](https://docs.flutter.dev/get-started/install) 3.47.7 (canal stable), com `flutter/bin` no PATH.
2. [Rust via rustup](https://rustup.rs). Depois, num terminal novo:
   ```sh
   rustup toolchain install stable
   rustup target add wasm32-unknown-unknown
   rustup toolchain install nightly --component rust-src
   rustup target add wasm32-unknown-unknown --toolchain nightly
   cargo install flutter_rust_bridge_codegen --version 2.13.0 --locked
   cargo install wasm-pack --locked
   cargo install wasm-bindgen-cli --version 0.2.129 --locked
   cargo install cargo-deny --locked
   ```
3. Python 3 (só para servir a build web localmente).

### Windows

- Visual Studio 2022 ou mais novo, com a carga de trabalho **"Desenvolvimento para desktop com C++"**.
- **Modo de Desenvolvedor ligado** (Configurações › Privacidade e segurança › Para desenvolvedores).
  O Flutter precisa dele para criar links simbólicos ao compilar apps com plugins.
  Atalho: `start ms-settings:developers`.
- Se houver um Dart avulso instalado, `flutter\bin` precisa vir **antes** dele no PATH
  (`dart --version` deve mostrar a mesma versão do Flutter).

### macOS

- Xcode (com as ferramentas de linha de comando) e CocoaPods: `sudo gem install cocoapods`.

### Linux (Ubuntu/Debian)

```sh
sudo apt install clang cmake ninja-build pkg-config libgtk-3-dev liblzma-dev libstdc++-12-dev
```

Confira tudo com `flutter doctor`. Só precisam aparecer como ok o seu desktop e o Chrome; Android e iOS não são alvos.

## Compilar e rodar

### Testes do núcleo (qualquer sistema)

```sh
cd core
cargo test
cargo clippy --all-targets -- -D warnings
```

### Desktop

O `flutter run` compila o núcleo Rust automaticamente (cargokit):

```sh
cd app
flutter run -d windows   # ou: -d macos, -d linux
```

A saída vai para a pasta `Morsa` dentro de **Documentos**. O arquivo original nunca é alterado.
Se o nome já existir, a saída ganha um sufixo, como `foto (1).jpg`.

### Web

A build web não funciona aberta direto do disco (`file://`); ela precisa de um servidor
local. Use o script do projeto, que também envia os cabeçalhos COOP/COEP exigidos pelos
workers do núcleo:

```sh
# Windows (PowerShell)
./scripts/build_web.ps1
# Linux / macOS
./scripts/build_web.sh

python scripts/serve_web.py      # abre em http://localhost:8080
```

Para desenvolver com recarga rápida:

```sh
cd app
dart run flutter_rust_bridge build-web --dart-root . --rust-root ../core --output "$(pwd)/web" --release
flutter run -d chrome --web-header=Cross-Origin-Opener-Policy=same-origin --web-header=Cross-Origin-Embedder-Policy=require-corp
```

### Testes e análise do app

```sh
cd app
flutter analyze
flutter test
```

## Quando mudar…

| …isto | rode (na raiz, salvo indicação) |
| --- | --- |
| funções ou tipos em `core/src/api.rs` | `cd app && flutter_rust_bridge_codegen generate` |
| textos em `app/l10n/*.arb` | `cd app && flutter gen-l10n` |
| cores do sistema visual | edite `scripts/gen_tokens.dart` e rode `dart run scripts/gen_tokens.dart` |
| o ícone `assets/icon.svg` | `cargo run --release --manifest-path scripts/icons/Cargo.toml` |
| o gerador do corpus | `cd core && cargo run --example gen_corpus` |

## Privacidade e rede

O Morsa não faz nenhuma requisição de rede em execução: sem telemetria, sem
verificação de versão, sem fontes ou recursos de CDN. As fontes (IBM Plex Sans
Condensed e Ioskeley Mono NL, ambas OFL-1.1) vêm empacotadas em `app/assets/fonts`.
Na web, a Content Security Policy só permite a própria origem. O teste
automatizado de zero rede entra no Marco 2.

## Licença

O código está sob [MIT](LICENSE-MIT) ou [Apache-2.0](LICENSE-APACHE), à sua escolha.
Contribuições são aceitas sob os mesmos termos. As fontes têm licença própria (OFL-1.1),
com os textos em `app/assets/fonts/*/OFL.txt`.
