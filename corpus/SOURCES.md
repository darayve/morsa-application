# Origem dos arquivos do corpus (TST09)

Todos os arquivos abaixo foram **gerados pelo próprio projeto** e estão sob a
licença do Morsa (MIT OR Apache-2.0). Nenhum vem de terceiros. Para recriá-los
(o resultado é idêntico, byte a byte; a CI confere isso):

```
cd core
cargo run --example gen_corpus
```

Os arquivos válidos são gravados por codificadores **independentes do núcleo**
(`image`, `gif`, `tiff`, `jpeg-encoder`), para que ler com o núcleo seja um
teste de verdade. Limitação conhecida: são os mesmos crates que o núcleo usa
para ler, então um erro de interpretação comum aos dois lados passaria batido;
arquivos reais de câmeras e editores entram quando houver licença livre
registrada aqui.

Os testes que usam o corpus ficam em `core/tests/` (`image_matrix.rs`,
`image_cases.rs`, `image_errors.rs`) e em `app/integration_test/`.

## Casos que valem para todos os formatos

Cada pasta (`png/`, `jpg/`, `webp/`, `gif/`, `bmp/`, `tiff/`, `ico/`, `svg/`) tem:

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `minimo.*` | O menor arquivo válido (1 × 1; no ICO, um quadro de 16 × 16) | Converte para todos os destinos |
| `tipico.*` (`valido.*`, `multitamanho.ico`, `simples.svg`) | Imagem típica de 64 × 48 com degradê e xadrez de luminância | Converte; pixels conferidos (TST05) |
| `truncado.*` | Os primeiros 60% de um arquivo válido | `E-105` |
| `cabecalho-invalido.*` | Assinatura correta seguida de lixo | `E-105` (GIF: `E-106`; BMP: `E-102`) |

## Arquivos de cada formato

### PNG (`image/png/`)

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `valido-rgba.png` | 64 × 48 RGBA; metade direita transparente, coluna central com 50% de alfa | Transparência vira o fundo escolhido em JPG/BMP |
| `valido-16bit.png` | 32 × 32 com 16 bits por canal | Reduzido com arredondamento para 8 bits; preservado em TIFF |
| `cinza.png`, `cinza-alfa.png` | 32 × 32 em tons de cinza, sem e com alfa | Continua cinza (JPG, TIFF) |
| `icc-p3.png` | Cor Display P3 (200, 100, 50) com perfil ICC | Vira sRGB ≈ (215, 93, 31) |
| `grande.png` | 3000 × 2000 (6 MP) | Converte para JPG, WebP e TIFF |
| `png-extensao-errada.jpg` | PNG válido com extensão `.jpg` | Tratado como PNG (RN14) |
| `nao-e-png.png` | Texto simples com extensão `.png` | `E-102` |
| `vazio.png` | 0 bytes | `E-103` |
| `dimensoes-gigantes.png` | 65 bytes que declaram 40 000 × 40 000 px | `E-106`, sem decodificar |

### JPG (`image/jpg/`)

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `cinza.jpg` | 1 canal | Continua cinza |
| `cmyk.jpg` | Faixas de ciano, magenta, amarelo e preto puros | Converte para RGB |
| `icc-p3.jpg` | Cor Display P3 com perfil ICC | Vira sRGB |
| `exif-1.jpg` … `exif-8.jpg` | Mesma cena guardada deitada/espelhada/girada, com a etiqueta de orientação EXIF 1–8 | Sai na posição certa (igual a `exif-canonico.png`) |
| `exif-canonico.png` | A cena "como deve ser vista": 4 quadrantes coloridos e um quadrado branco | Referência dos testes de orientação |
| `grande.jpg` | 3000 × 2000 | Converte para PNG, WebP e TIFF |
| `jpg-extensao-errada.png` | JPG válido com extensão `.png` | Tratado como JPG (RN14) |

### WebP (`image/webp/`)

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `transparente.webp` | Sem perdas, com alfa | Alfa preservado |
| `com-perdas.webp` | VP8 (com perdas) | Converte |
| `animado.webp` | 2 quadros (vermelho, azul) | 1º quadro + aviso `E-108` |

### GIF (`image/gif/`)

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `transparente.gif` | Metade transparente | Alfa preservado |
| `animado.gif` | 3 quadros (vermelho, verde, azul) | 1º quadro + aviso `E-108` |

### TIFF (`image/tiff/`)

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `transparente.tiff` | RGBA com alfa não associado | Alfa preservado |
| `16bit.tiff` | 16 bits por canal | Preservado em PNG |
| `multipagina.tiff` | 2 páginas (vermelha, azul) | 1ª página + aviso `E-109` |

### ICO (`image/ico/`)

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `multitamanho.ico` | 3 quadros: 16 px vermelho, 32 px verde, 48 px azul | Usa o maior (48 px, azul) |

### SVG (`image/svg/`)

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `texto.svg` | Texto "Morsa" | Renderizado com a fonte empacotada |
| `so-viewbox.svg` | Só `viewBox`, sem `width`/`height` | 1024 px de largura (RF06) |
| `referencias-externas.svg` | Aponta para uma imagem na internet e para um arquivo do disco | Nada é buscado; aviso `E-110` |
| `dimensoes-gigantes.svg` | Declara 100 000 × 100 000 px | `E-106` |
