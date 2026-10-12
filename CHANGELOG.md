# Changelog

Formato: [Keep a Changelog](https://keepachangelog.com/pt-BR/1.1.0/). Versionamento semântico.

## [Não lançado]

### Adicionado
- Esqueleto do repositório: núcleo Rust (`core/`), app Flutter (`app/`), ponte flutter_rust_bridge.
- Conversão PNG → JPG ponta a ponta no desktop e na web.
- Integração contínua no GitHub Actions: formatação, análise, testes do núcleo em Windows,
  macOS e Linux, cobertura, build WASM, builds desktop e testes de integração.
- Política de dependências: `cargo deny` (licenças, clientes de rede, vulnerabilidades) e
  verificação equivalente para os pacotes Dart.
- Teste automatizado de zero rede na web (Playwright) e no desktop (Linux, rede isolada).
- Fuzzing do conversor de imagens (5 min por push, 1 h por dia).
- Componentes base do sistema visual e galeria de componentes para desenvolvimento.

### Corrigido
- Botões agora podem ser acionados por leitores de tela.
- macOS: sem App Sandbox, que impediria gravar em Documentos/Morsa (DC14).
