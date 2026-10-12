# Origem dos arquivos do corpus (TST09)

Todos os arquivos abaixo foram **gerados pelo próprio projeto** e estão sob a
licença do Morsa (MIT OR Apache-2.0). Para recriá-los (o resultado é idêntico):

```
cd core
cargo run --example gen_corpus
```

| Arquivo | O que é | Resultado esperado |
| --- | --- | --- |
| `image/png/valido-rgba.png` | PNG 64 × 48 RGBA; metade direita transparente, coluna central com 50% de alfa | Converte; transparência vira branco no JPG |
| `image/png/valido-16bit.png` | PNG 32 × 32 com 16 bits por canal | Converte, reduzido para 8 bits |
| `image/png/png-extensao-errada.jpg` | O mesmo PNG válido, salvo com extensão `.jpg` | Tratado como PNG (RN14) |
| `image/png/truncado.png` | Os primeiros 60% do PNG válido | `E-105` |
| `image/png/cabecalho-invalido.png` | Assinatura PNG seguida de texto | `E-105` |
| `image/png/nao-e-png.png` | Texto simples com extensão `.png` | `E-102` |
| `image/png/vazio.png` | 0 bytes | `E-103` |
| `image/png/dimensoes-gigantes.png` | 65 bytes que declaram 40 000 × 40 000 px | `E-106`, sem decodificar |
| `image/jpg/valido.jpg` | JPG 40 × 30 | JPG → JPG sem transformação dá `E-101` |
