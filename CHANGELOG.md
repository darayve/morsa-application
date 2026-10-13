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

- Conversão de imagens entre PNG, JPG, WebP, GIF, BMP, TIFF e ICO, e SVG como entrada (49 pares da matriz),
  com opções de qualidade, modo do WebP, cor de fundo, tamanhos do ICO, compressão do TIFF e tamanho do SVG.
- Tratamento de transparência, 16 bits, CMYK, perfil de cor (para sRGB), orientação EXIF, animações,
  TIFF multipágina e ICO multitamanho; avisos E-108, E-109 e E-110.
- Corpus de teste com cerca de 70 arquivos, válidos e corrompidos, gerados pelo projeto.

### Corrigido
- Botões agora podem ser acionados por leitores de tela.
- macOS: sem App Sandbox, que impediria gravar em Documentos/Morsa (DC14).
- JPG truncado agora é recusado com E-105 (o decodificador devolvia metade da imagem em silêncio).
