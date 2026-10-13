# Morsa — Documento de Requisitos

| Campo | Valor |
| --- | --- |
| Produto | Morsa |
| Documento | Especificação de requisitos de software (ERS) |
| Versão do documento | 1.0 |
| Data | 2026-10-12 |
| Situação | Aprovado para início do desenvolvimento (MVP) |
| Licença do projeto | MIT OR Apache-2.0 |

---

## Sumário

0. [Como ler este documento](#0-como-ler-este-documento)
1. [Visão do produto](#1-visão-do-produto)
2. [Glossário](#2-glossário)
3. [Escopo por fase](#3-escopo-por-fase)
4. [Catálogo de ferramentas](#4-catálogo-de-ferramentas)
5. [Formatos suportados](#5-formatos-suportados)
6. [Regras de negócio](#6-regras-de-negócio)
7. [Requisitos funcionais](#7-requisitos-funcionais)
8. [Fluxos de uso](#8-fluxos-de-uso)
9. [Interface e design](#9-interface-e-design)
10. [Requisitos não funcionais](#10-requisitos-não-funcionais)
11. [Segurança e privacidade](#11-segurança-e-privacidade)
12. [Arquitetura técnica](#12-arquitetura-técnica)
13. [Dependências e licenciamento](#13-dependências-e-licenciamento)
14. [Qualidade e testes](#14-qualidade-e-testes)
15. [Integração contínua e entrega](#15-integração-contínua-e-entrega)
16. [Distribuição](#16-distribuição)
17. [Projeto de código aberto](#17-projeto-de-código-aberto)
18. [Plano de desenvolvimento](#18-plano-de-desenvolvimento)
19. [Riscos](#19-riscos)
20. [Registro de decisões e pendências](#20-registro-de-decisões-e-pendências)
- [Apêndice A — Catálogo de mensagens de erro](#apêndice-a--catálogo-de-mensagens-de-erro)
- [Apêndice B — Atalhos de teclado](#apêndice-b--atalhos-de-teclado)
- [Apêndice C — Regras de nomes de saída](#apêndice-c--regras-de-nomes-de-saída)
- [Apêndice D — Assinaturas de formato (magic bytes)](#apêndice-d--assinaturas-de-formato-magic-bytes)

---

## 0. Como ler este documento

### 0.1 Palavras normativas

| Termo | Significado |
| --- | --- |
| **DEVE** / **NÃO DEVE** | Obrigatório. Um item que não cumpre isso não está pronto. |
| **DEVERIA** / **NÃO DEVERIA** | Fortemente recomendado. Desvios precisam de justificativa registrada. |
| **PODE** | Opcional. |

### 0.2 Identificadores

| Prefixo | Tipo |
| --- | --- |
| `RN` | Regra de negócio |
| `RF` | Requisito funcional |
| `RNF` | Requisito não funcional |
| `SEG` | Requisito de segurança |
| `TST` | Requisito de teste |
| `E-` | Código de erro exibido ao usuário |
| `IMG-`, `PDF-`, `ZIP-`, `CLR-`, `SYS-` | Códigos de ferramentas do catálogo |

Identificadores nunca são reaproveitados. Um requisito removido mantém o ID marcado como **removido**.

### 0.3 Marcação de valores propostos

Valores marcados com **[P]** (proposto) são padrões definidos neste documento sem discussão prévia específica: limites numéricos, valores padrão de opções, detalhes de comportamento. Eles valem para a implementação, mas podem ser ajustados sem reabrir decisões de escopo. Itens sem **[P]** foram decididos explicitamente.

### 0.4 Prioridade (MoSCoW)

| Sigla | Significado |
| --- | --- |
| **M** | Must — obrigatório na fase indicada |
| **S** | Should — importante, mas pode escorregar para a fase seguinte |
| **C** | Could — desejável |
| **W** | Won't (nesta fase) — registrado para o futuro |

---

## 1. Visão do produto

### 1.1 Problema

Profissionais que recebem arquivos de fornecedores, clientes e parceiros (imagens em formatos variados, PDFs, pacotes ZIP e RAR) precisam convertê-los, juntá-los, separá-los ou extraí-los com frequência. Hoje a solução mais comum são conversores online, o que significa enviar arquivos potencialmente confidenciais (contratos, projetos, plantas, fotos de produto não lançado) para servidores de terceiros, de propriedade desconhecida, com políticas de retenção desconhecidas. A alternativa é instalar vários programas diferentes, cada um para uma tarefa.

### 1.2 Proposta

O Morsa é um único app de utilitários para arquivos que:

- roda **100% offline** em Windows, macOS, Linux e no navegador;
- **não exige conta**, cadastro ou login;
- **não envia nenhum arquivo** para servidor algum, porque não existe servidor;
- é de **código aberto**, para que qualquer pessoa possa auditar, clonar, compilar e modificar;
- reúne **várias ferramentas em um só lugar**, com uma interface de ferramenta profissional.

O nome vem da morsa, a ferramenta de bancada que prende uma peça com firmeza: os arquivos do usuário ficam presos ao computador dele.

### 1.3 Personas

**Ana, arquiteta (usuária principal).** Recebe plantas em PDF, renders em TIFF e fotos de obra em vários formatos. Precisa juntar PDFs de fornecedores num único arquivo para o cliente e converter imagens para o formato que o software dela aceita. Tem receio de subir projetos de clientes em sites desconhecidos. Não é técnica: quer arrastar, clicar e pronto.

**Bruno, analista de compras.** Recebe catálogos de fornecedores como ZIP e RAR, com imagens de produtos em formatos mistos. Precisa extrair, converter tudo para JPG ou WebP e mandar para o time de e-commerce. Trabalha num computador corporativo onde instalar programas é burocrático; a versão web, que roda no navegador sem instalação, é valiosa para ele.

**Carla, desenvolvedora (contribuidora).** Conhece o projeto pelo GitHub. Quer auditar que o app realmente não usa rede, compilar a própria versão e eventualmente adicionar uma ferramenta nova. Precisa de um repositório organizado, build reproduzível e documentação de arquitetura.

### 1.4 Princípios do produto

| # | Princípio | Consequência prática |
| --- | --- | --- |
| P1 | **Zero rede** | Nenhuma requisição de rede em tempo de execução, nem para fontes, atualizações ou telemetria. |
| P2 | **Zero conta** | Nenhuma identificação do usuário em nenhuma circunstância. |
| P3 | **Zero servidor** | Todo processamento acontece no dispositivo do usuário. |
| P4 | **Código aberto** | Qualquer pessoa pode clonar, compilar e modificar. |
| P5 | **Tudo em um lugar** | Novas ferramentas entram no mesmo app, no mesmo catálogo. |
| P6 | **Fatos, não selos** | A interface comunica privacidade por fatos verificáveis ("Sem acesso à rede"), nunca por promessas de marketing ("100% seguro"). |
| P7 | **Original intocado** | O app nunca altera o arquivo de entrada. |
| P8 | **Nada escondido** | Todo arquivo ignorado ou com falha aparece com o motivo. |

### 1.5 Não objetivos

O Morsa **não** é, e não deve se tornar:

- um compressor ou "otimizador" de tamanho de arquivos;
- um editor de imagens (além de redimensionar, rotacionar e recortar);
- um gerenciador de arquivos;
- um serviço com qualquer componente em nuvem;
- um produto com planos pagos, anúncios ou coleta de dados.

---

## 2. Glossário

| Termo | Definição |
| --- | --- |
| **Ferramenta** | Uma função do app com código no catálogo (ex.: `IMG-01 Converter imagens`). Na interface é sempre chamada de "ferramenta". |
| **Ordem** | Uma execução de uma ferramenta sobre um conjunto de arquivos. Recebe número sequencial local (`OS-0001`), exibido como detalhe secundário. |
| **Lote** | Conjunto de arquivos processados numa mesma ordem. |
| **Item** | Cada arquivo dentro de um lote, numerado `001`, `002`… |
| **Triagem** | Etapa que agrupa arquivos de tipos misturados e propõe uma ferramenta para cada grupo. |
| **Histórico** | Registro local das ordens executadas. |
| **Preset** | Conjunto salvo de opções de uma ferramenta. |
| **Núcleo** | Biblioteca em Rust que faz todo o processamento de arquivos (`core/`). |
| **Ponte** | Camada gerada pelo `flutter_rust_bridge` que conecta a interface Flutter ao núcleo Rust. |
| **WASM** | WebAssembly: formato que permite rodar o núcleo Rust dentro do navegador. |
| **Web Worker** | Thread separada do navegador onde o núcleo roda na versão web, para não travar a interface. |
| **PWA** | Progressive Web App: site instalável que funciona offline após o primeiro acesso. |
| **Rasterizar** | Converter uma imagem vetorial (SVG) em pixels, numa resolução escolhida. |
| **Metadados** | Informações embutidas no arquivo além do conteúdo: EXIF, GPS, autor, software etc. |
| **Magic bytes** | Sequência de bytes no início de um arquivo que identifica o formato real, independente da extensão. |
| **Zip bomb** | Arquivo compactado pequeno que se expande para um tamanho enorme, para esgotar disco ou memória. |
| **Zip slip** | Ataque em que um item dentro de um arquivo compactado tem caminho como `../../x`, tentando escrever fora da pasta de destino. |
| **Decompression bomb** | Imagem pequena em bytes que declara dimensões gigantescas, para esgotar memória ao decodificar. |
| **Codec** | Algoritmo que comprime e descomprime um tipo de dado. |
| **Corpus** | Coleção de arquivos reais (válidos e problemáticos) usada nos testes. |

---

## 3. Escopo por fase

### 3.1 MVP

| Área | Inclui |
| --- | --- |
| Imagens | Conversão entre PNG, JPG, WebP, GIF, BMP, TIFF e ICO; SVG como entrada; edição básica (redimensionar, rotacionar, recortar); lote |
| PDF | PDF para imagem; imagens para PDF; juntar; dividir; reordenar páginas |
| Compactados | ZIP: ver conteúdo, extrair, criar |
| Transversal | Triagem de tipos mistos, presets, histórico, configurações, pt-BR e inglês, temas claro e escuro, acessibilidade |
| Plataformas | Windows, macOS, Linux, web (PWA) |

### 3.2 Pós-MVP

| Área | Inclui |
| --- | --- |
| Compactados | 7z, TAR, GZ e XZ (extrair e criar); RAR (somente extrair); conversão entre formatos compactados |

### 3.3 Futuro

| Área | Inclui |
| --- | --- |
| Cores | `CLR-01` Seletor de cores |
| PDF | `PDF-06` Edição de PDF |

### 3.4 Fora de escopo (decidido)

- Compressão ou redução de tamanho de arquivos, inclusive compressão de PDF e "otimização" de imagens.
- HEIC/HEIF (licença LGPL da única implementação madura e patentes ativas do codec HEVC).
- Criação de arquivos RAR (formato proprietário).
- Documentos de escritório (DOCX, XLSX, PPTX), áudio, vídeo, OCR.
- Vetorização de imagens (pixels para SVG).
- GIF animado e WebP animado como saída; animação é descartada na entrada.
- Atualização automática pela rede.
- Qualquer funcionalidade que dependa de rede, conta ou servidor.

---

## 4. Catálogo de ferramentas

### 4.1 Regra de codificação

- Código de ferramenta: `CAT-NN`, onde `CAT` é a categoria (3 letras) e `NN` é sequencial a partir de `01`, na ordem de entrada no catálogo.
- `-00` é reservado ao próprio sistema (`SYS-00 Configurações`).
- Códigos nunca são reaproveitados, mesmo se a ferramenta for removida.
- Ferramentas futuras recebem código na categoria de destino e estado "Em breve".
- Ordens: `OS-NNNN`, sequencial local por instalação. Itens: `001`, `002`…
- Páginas de PDF nas telas de juntar e reordenar: letra do arquivo de origem + número da página (`A1`, `B6`), onde a letra segue a ordem de entrada do arquivo.

### 4.2 Catálogo

| Código | Ferramenta | Categoria | Fase | Estado inicial |
| --- | --- | --- | --- | --- |
| `IMG-01` | Converter imagens | Imagens | MVP | Disponível |
| `PDF-01` | PDF → imagem | PDF | MVP | Disponível |
| `PDF-02` | Imagens → PDF | PDF | MVP | Disponível |
| `PDF-03` | Juntar PDFs | PDF | MVP | Disponível |
| `PDF-04` | Dividir PDF | PDF | MVP | Disponível |
| `PDF-05` | Reordenar páginas | PDF | MVP | Disponível |
| `ZIP-01` | Ver conteúdo | Compactados | MVP | Disponível |
| `ZIP-02` | Extrair | Compactados | MVP | Disponível |
| `ZIP-03` | Criar ZIP | Compactados | MVP | Disponível |
| `ZIP-04` | Converter compactado | Compactados | Pós-MVP | Em breve |
| `PDF-06` | Editar PDF | PDF | Futuro | Em breve |
| `CLR-01` | Seletor de cores | Cores | Futuro | Em breve |
| `SYS-00` | Configurações | Sistema | MVP | Fixo no rodapé do índice |

Ferramentas "Em breve" aparecem no índice em cinza, não são executáveis e exibem o texto "Em breve" ao serem focadas **[P]**.

### 4.3 Categorias

| Prefixo | Nome na interface | Cor de etiqueta (token) |
| --- | --- | --- |
| `IMG` | Imagens | `--img` |
| `PDF` | Documentos PDF | `--pdf` |
| `ZIP` | Compactados | `--zip` |
| `CLR` | Cores | a definir quando lançada |
| — | Em breve | `--fut` |

Uma categoria nova entra como mais um bloco no índice (etiqueta, prefixo e ferramentas), sem mudar a estrutura da interface. A etiqueta toma a próxima matiz livre com luminosidade e croma próximos aos das existentes.

---

## 5. Formatos suportados

### 5.1 Matriz de formatos

| Formato | Extensões aceitas | Entrada | Saída | Fase | Observações |
| --- | --- | --- | --- | --- | --- |
| PNG | `.png` | Sim | Sim | MVP | Sem perdas; preserva transparência e 16 bits por canal |
| JPG | `.jpg`, `.jpeg`, `.jpe`, `.jfif` | Sim | Sim | MVP | Com perdas; sem transparência; opção de qualidade |
| WebP | `.webp` | Sim | Sim | MVP | Com ou sem perdas; transparência; animado: só 1º quadro na entrada |
| GIF | `.gif` | Sim | Sim | MVP | Entrada: só 1º quadro. Saída: estática, paleta de 256 cores |
| BMP | `.bmp`, `.dib` | Sim | Sim | MVP | Sem transparência na saída |
| TIFF | `.tif`, `.tiff` | Sim | Sim | MVP | Multipágina: só 1ª página na entrada **[P]**, com aviso |
| ICO | `.ico` | Sim | Sim | MVP | Entrada: maior quadro. Saída: multitamanho |
| SVG | `.svg` | Sim | Não | MVP | Rasterizado no tamanho escolhido |
| PDF | `.pdf` | Sim | Sim | MVP | Ver operações em 5.4 |
| ZIP | `.zip` | Sim | Sim | MVP | Ver 5.5 |
| 7z | `.7z` | Sim | Sim | Pós-MVP | |
| TAR | `.tar` | Sim | Sim | Pós-MVP | |
| GZ | `.gz`, `.tgz`, `.tar.gz` | Sim | Sim | Pós-MVP | |
| XZ | `.xz`, `.txz`, `.tar.xz` | Sim | Sim | Pós-MVP | |
| RAR | `.rar` | Sim | Não | Pós-MVP | Somente extração |

### 5.2 Conversões de imagem permitidas

Toda imagem de entrada pode ser convertida para qualquer formato de imagem de saída e também para PDF (`PDF-02`). A conversão para o mesmo formato de origem é permitida quando houver alguma transformação (redimensionar, rotacionar, recortar, remover metadados ou mudar qualidade); sem transformação, a interface não oferece o mesmo formato como destino **[P]**.

| Entrada ↓ / Saída → | PNG | JPG | WebP | GIF | BMP | TIFF | ICO | PDF |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| PNG | ◐ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| JPG | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |
| WebP | ✓ | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ | ✓ |
| GIF | ✓ | ✓ | ✓ | ◐ | ✓ | ✓ | ✓ | ✓ |
| BMP | ✓ | ✓ | ✓ | ✓ | ◐ | ✓ | ✓ | ✓ |
| TIFF | ✓ | ✓ | ✓ | ✓ | ✓ | ◐ | ✓ | ✓ |
| ICO | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ◐ | ✓ |
| SVG | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ | ✓ |

✓ permitido · ◐ permitido apenas com transformação

### 5.3 Tratamento de características de imagem

| Situação | Comportamento |
| --- | --- |
| Transparência → formato sem transparência (JPG, BMP) | Compor sobre a cor de fundo escolhida (padrão: branco) |
| Transparência → GIF | Transparência binária: pixels com alfa < 50% viram transparentes **[P]** |
| Transparência → ICO, PNG, WebP, TIFF | Preservada |
| 16 bits por canal → formato de 8 bits | Reduzir para 8 bits com arredondamento |
| 16 bits → PNG ou TIFF | Preservado |
| Mais de 256 cores → GIF | Quantização para paleta de 256 cores com pontilhamento (dithering) **[P]** |
| Orientação EXIF | **DEVE** ser aplicada fisicamente aos pixels **antes** de remover metadados, para a imagem não sair girada |
| CMYK (comum em JPG de gráfica) | Converter para RGB (sRGB) **[P]** |
| Perfil de cor ICC | Converter para sRGB **[P]**; ver risco R7 |
| Imagem animada (GIF, WebP) | Usar apenas o 1º quadro e avisar "Somente o primeiro quadro foi usado" |
| TIFF multipágina | Usar apenas a 1ª página e avisar **[P]** |
| ICO com vários tamanhos | Usar o maior quadro |
| SVG com referências externas (imagens, fontes por URL) | **NÃO DEVE** buscar nada externo; referências externas são ignoradas, com aviso |
| SVG com texto | Renderizar com as fontes empacotadas no app; fontes do sistema **PODEM** ser usadas no desktop **[P]** |

### 5.4 Operações de PDF

| Código | Operação | Entrada | Saída |
| --- | --- | --- | --- |
| `PDF-01` | PDF → imagem | 1 ou mais PDFs | 1 imagem por página (PNG, JPG ou WebP) |
| `PDF-02` | Imagens → PDF | 1 ou mais imagens | 1 PDF (1 imagem por página) |
| `PDF-03` | Juntar | 2 ou mais PDFs | 1 PDF |
| `PDF-04` | Dividir | 1 PDF | 2 ou mais PDFs |
| `PDF-05` | Reordenar páginas | 1 PDF | 1 PDF com páginas reordenadas, giradas ou removidas |

### 5.5 Operações com compactados

| Código | Operação | Formatos (MVP) | Formatos (Pós-MVP) |
| --- | --- | --- | --- |
| `ZIP-01` | Ver conteúdo | ZIP | 7z, TAR, GZ, XZ, RAR |
| `ZIP-02` | Extrair | ZIP | 7z, TAR, GZ, XZ, RAR |
| `ZIP-03` | Criar | ZIP | 7z, TAR, GZ, XZ |
| `ZIP-04` | Converter entre formatos | — | qualquer entrada → ZIP, 7z, TAR, GZ, XZ |

### 5.6 Arquivos que parecem ZIP mas não são pacotes

Formatos como DOCX, XLSX, PPTX, ODT, EPUB e JAR são internamente arquivos ZIP. O app **DEVE** identificá-los como "documento não suportado" (pela presença de `[Content_Types].xml`, `mimetype` ou `META-INF/MANIFEST.MF` na raiz) e **NÃO DEVE** oferecê-los às ferramentas de compactados **[P]**. Motivo: o usuário que solta uma planilha não quer "extrair" a planilha.

---

## 6. Regras de negócio

### RN01 — Matriz de conversão é a única fonte de verdade
Só são permitidas conversões previstas na seção 5. A interface **NÃO DEVE** oferecer destino inválido; o núcleo **DEVE** recusar com erro `E-101` qualquer pedido fora da matriz, mesmo que a interface falhe.

### RN02 — O original nunca é alterado
O arquivo de entrada **NÃO DEVE** ser modificado, renomeado, movido ou apagado em nenhuma circunstância. Toda saída é um arquivo novo. Isso vale inclusive quando o destino escolhido é a mesma pasta da origem.

### RN03 — Nunca sobrescrever sem confirmação
Se já existir um arquivo com o nome de saída, o app adiciona sufixo numérico: `foto.png` → `foto (1).png` → `foto (2).png`. Sobrescrever só acontece se o usuário escolher explicitamente "Substituir" num diálogo de conflito **[P]**; por padrão, o comportamento é sempre o sufixo. Ver Apêndice C.

### RN04 — Falha isolada no lote
A falha de um item não interrompe os demais. Ao final, o resumo mostra quantos itens concluíram, falharam e foram ignorados, e cada falha tem o motivo em linguagem simples e a sugestão de ação (Apêndice A).

### RN05 — Proteções de senha são respeitadas
PDFs e ZIPs protegidos por senha só são processados se o usuário informar a senha. O app **NÃO DEVE** tentar adivinhar, quebrar ou remover proteções. Quando o usuário informa a senha e a operação gera um novo arquivo (ex.: juntar um PDF protegido com outros), a saída sai **sem** senha **[P]**, e isso **DEVE** ser avisado antes da execução.

### RN06 — Sem limites de negócio
Não existem limites de tamanho, quantidade ou uso por regra de negócio. Os únicos limites são técnicos: memória disponível, espaço em disco e os limites de segurança da RN10 e da SEG06.

### RN07 — Metadados removidos por padrão
Na conversão de imagens, metadados (EXIF, GPS, XMP, IPTC, comentários, autor, software) são removidos por padrão. O usuário **PODE** marcar "Manter metadados". A orientação EXIF é sempre aplicada aos pixels antes da remoção. O perfil de cor é tratado conforme 5.3, não como metadado comum. Em PDF, o MVP preserva os metadados do documento (título, autor) nas operações de juntar, dividir e reordenar **[P]**.

### RN08 — RAR somente para extração
O app nunca cria arquivos RAR. RAR é aceito apenas como entrada (Pós-MVP).

### RN09 — Extração confinada à pasta de destino
Nenhum item extraído pode ser escrito fora da pasta de destino escolhida. Caminhos absolutos, caminhos com `..`, letras de unidade (`C:`), caminhos UNC (`\\servidor`) e links simbólicos que apontem para fora são rejeitados item a item, com aviso, sem interromper a extração dos demais (`E-305`).

### RN10 — Limites de segurança na extração
A extração respeita os limites abaixo, contando os bytes **realmente extraídos**, nunca o tamanho declarado no cabeçalho do arquivo. Ao passar de um limite, a extração para e o conteúdo parcialmente extraído é apagado.

| Limite | Valor |
| --- | --- |
| Tamanho total extraído (desktop) | 10 GB, ajustável em Configurações |
| Tamanho total extraído (web) | 1 GB |
| Espaço livre em disco (desktop) | Manter pelo menos 1 GB livre |
| Taxa de compressão por arquivo | Acima de 1000:1, pedir confirmação ao usuário |
| Quantidade de itens | 100 mil |
| Compactados dentro de compactados | Nunca extrair automaticamente |

### RN11 — Nada sai do dispositivo
Nenhum arquivo, dado, metadado ou informação de uso sai do dispositivo do usuário, por nenhum meio.

### RN12 — O histórico guarda descrições, nunca conteúdo [P]
O histórico guarda: número da ordem, ferramenta, quantidade de itens, formatos de entrada e saída, situação, data e hora, mensagens de falha e nomes dos arquivos. **NÃO DEVE** guardar conteúdo de arquivos, miniaturas, senhas ou caminhos completos de pasta. O usuário **PODE** desligar o registro de nomes de arquivos e **PODE** limpar o histórico a qualquer momento.

### RN13 — Senhas nunca são guardadas [P]
Senhas informadas para PDF ou ZIP existem apenas em memória durante a operação, nunca são gravadas em disco, histórico, presets ou logs, e são descartadas ao final da ordem.

### RN14 — O formato real manda, não a extensão
O formato de um arquivo é determinado pelo conteúdo (magic bytes, Apêndice D). Se a extensão não corresponder ao conteúdo (ex.: um PNG salvo como `.jpg`), o app trata o arquivo pelo formato real e exibe um aviso informativo ("Este arquivo é PNG, apesar da extensão .jpg") **[P]**.

### RN15 — Extensão de saída pelo formato de destino
A extensão do arquivo de saída é sempre a extensão padrão do formato de destino, em minúsculas: `.png`, `.jpg`, `.webp`, `.gif`, `.bmp`, `.tif` **[P]**, `.ico`, `.pdf`, `.zip`.

### RN16 — Numeração de ordens local e contínua [P]
A numeração `OS-NNNN` é local a cada instalação (desktop) ou a cada navegador (web), começa em `OS-0001` e nunca retrocede, mesmo que o histórico seja limpo. Passando de `OS-9999`, a numeração continua com mais dígitos (`OS-10000`).

---

## 7. Requisitos funcionais

Cada requisito traz prioridade (M/S/C/W), fase e critérios de aceitação verificáveis.

### 7.1 Entrada de arquivos

#### RF01 — Adicionar arquivos (M · MVP)
O usuário adiciona arquivos por: (a) seletor de arquivos do sistema; (b) arrastar e soltar em qualquer ponto da janela; (c) colar da área de transferência (arquivos copiados no gerenciador de arquivos ou imagem copiada de outro app).

**Critérios de aceitação**
- A janela inteira aceita arrastar e soltar, não apenas a área tracejada.
- Durante o arraste, a janela exibe a sobreposição de "soltar" (tela 1c, seção 9.6).
- Arraste de pastas (desktop): o conteúdo é incluído recursivamente **[P]**, ignorando arquivos ocultos e de sistema (`.DS_Store`, `Thumbs.db`, `desktop.ini`).
- Colar uma imagem crua da área de transferência gera um item chamado `colado-AAAAMMDD-HHMMSS.png` **[P]**.
- Na web, o seletor usa `<input type="file" multiple>`; arraste de pastas usa a API de diretórios quando disponível.
- Nenhum byte do arquivo é lido antes de o usuário soltar ou confirmar a seleção.

#### RF02 — Detectar formato pelo conteúdo (M · MVP)
O formato de cada arquivo é identificado pelos primeiros bytes (Apêndice D), com a extensão usada só como desempate (ex.: distinguir TAR, que não tem assinatura no início).

**Critérios de aceitação**
- Um PNG renomeado para `.jpg` é tratado como PNG e gera o aviso da RN14.
- Um arquivo sem assinatura reconhecível recebe "Formato não suportado" (`E-102`) e entra na lista de ignorados.
- A detecção lê no máximo os primeiros 4 KB **[P]**, exceto para SVG (até 64 KB, por ser texto) e para os casos da seção 5.6 (leitura do diretório central do ZIP).
- Arquivos de 0 bytes recebem `E-103` ("Arquivo vazio").

#### RF03 — Oferecer apenas destinos válidos (M · MVP)
Para cada item, o seletor de destino lista apenas os formatos permitidos pela matriz (5.2).

**Critérios de aceitação**
- Num lote misto (ex.: PNG e SVG), o seletor de destino mostra a interseção de destinos válidos.
- O próprio formato de origem só aparece se houver transformação ativa (5.2).
- O núcleo recusa destinos inválidos mesmo se a interface falhar (RN01).

#### RF04 — Processar em lote (M · MVP)
Todas as ferramentas aceitam vários arquivos por ordem (exceto `PDF-04` e `PDF-05`, que operam sobre 1 PDF por vez).

**Critérios de aceitação**
- Lote com formatos de entrada mistos converte todos para o mesmo destino.
- O destino e as opções podem ser definidos para o lote inteiro e sobrescritos por item **[P]**.
- Itens podem ser removidos do lote antes da execução (tecla `Del` ou botão de remover).
- O lote é processado em paralelo até o número de núcleos lógicos menos 1, no mínimo 1 **[P]** (desktop); na web, um worker por vez no MVP **[P]**.
- A falha de um item não interrompe os demais (RN04).

#### RF05 — Triagem de tipos misturados (M · MVP)
Quando os arquivos soltos pertencem a mais de uma categoria, o app abre a tela de triagem (tela 1d) em vez de uma ferramenta.

**Critérios de aceitação**
- Os arquivos são agrupados por categoria (Imagens, PDF, Compactados) e cada grupo recebe uma ferramenta proposta e um número de ordem.
- Regras de proposta: imagens → `IMG-01`; 2 ou mais PDFs → `PDF-03`; 1 PDF → `PDF-05`; 1 ou mais ZIPs → `ZIP-02`.
- O usuário pode trocar a ferramenta de cada grupo pelo painel "Ferramentas compatíveis" e incluir ou excluir grupos (barra de espaço).
- Arquivos sem ferramenta compatível aparecem numa linha própria com o motivo ("Nenhuma ferramenta aceita XLSX · ignorado"), nunca escondidos.
- Se todos os arquivos forem da mesma categoria, a triagem é pulada e a ferramenta proposta abre direto.
- "Abrir N ordens" abre as ordens como rascunhos, na ordem numérica, começando pela primeira.

### 7.2 Configuração e execução

#### RF06 — Opções por formato (M · MVP)

| Opção | Aplica-se a | Faixa | Padrão |
| --- | --- | --- | --- |
| Qualidade | JPG, WebP com perdas | 1–100 | 85 **[P]** |
| Modo WebP | WebP | Com perdas / sem perdas | Com perdas **[P]** |
| Cor de fundo | Saída sem transparência | Qualquer cor (seletor + hex) | Branco `#FFFFFF` |
| Tamanhos do ICO | ICO | Combinação de 16, 24, 32, 48, 64, 128, 256 | 16, 32, 48, 256 **[P]** |
| Compressão TIFF | TIFF | Nenhuma / LZW / Deflate | LZW **[P]** |
| Tamanho de rasterização | SVG | Largura e/ou altura em px (1–16 384), ou escala (0,1×–20×) | Tamanho declarado no SVG; se ausente, 1024 px de largura **[P]** |
| Resolução (DPI) | PDF → imagem | 72–600 | 150 **[P]** |
| Intervalo de páginas | PDF → imagem, Dividir | Expressão de intervalo (ver RF12) | Todas |
| Manter metadados | Imagens | Sim / Não | Não (RN07) |

**Critérios de aceitação**
- Campos numéricos fora da faixa exibem erro inline (estado "Erro" do sistema visual) e bloqueiam a execução.
- Opções que não se aplicam ao destino escolhido ficam ocultas, não desabilitadas **[P]**.
- O núcleo valida as mesmas faixas e recusa valores inválidos (`E-104`).

#### RF07 — Edição básica de imagem (M · MVP)
Antes de converter, o usuário pode redimensionar, rotacionar e recortar.

**Redimensionar**
- Modos: por largura, por altura, por largura e altura (com ou sem manter proporção), por porcentagem, por "caber em" caixa máxima.
- Por padrão, nunca ampliar além do tamanho original **[P]** (opção "Permitir ampliar" desmarcada).
- Filtro de reamostragem: Lanczos3 **[P]**.
- Limites: 1 a 16 384 px por lado **[P]**.

**Rotacionar**
- 90° horário, 90° anti-horário, 180°; espelhar horizontal e vertical. Sem rotação em ângulo livre no MVP **[P]**.

**Recortar**
- Recorte retangular interativo na pré-visualização, com proporções fixas opcionais: livre, 1:1, 4:3, 3:2, 16:9 e original **[P]**.
- Campos numéricos de x, y, largura e altura para precisão.

**Critérios de aceitação**
- Ordem de aplicação fixa: orientação EXIF → recortar → rotacionar → redimensionar **[P]**.
- Edições se aplicam por item; "Aplicar a todos" replica para o lote. Recorte em lote só é oferecido quando todos os itens têm as mesmas dimensões **[P]**.
- A pré-visualização reflete as edições antes da execução.

#### RF08 — Pré-visualização (S · MVP)
- Miniatura de cada item na lista e visualização ampliada do item selecionado.
- Pré-visualização da saída para JPG e WebP com perdas, gerada sob demanda com debounce de 300 ms **[P]** ao mudar a qualidade.
- Miniaturas geradas com no máximo 256 px no maior lado **[P]** e mantidas só em memória (RN12).
- Arquivos acima de 100 megapixels **[P]** mostram um marcador "Pré-visualização indisponível" em vez de decodificar.

#### RF09 — Progresso e cancelamento (M · MVP)
- Barra de progresso por item e total, com contagem ("3 de 5 itens") e estimativa de tempo restante após 2 segundos de execução **[P]**.
- Botão "Cancelar" (tecla `Esc`) interrompe a ordem: itens concluídos permanecem, o item em andamento é interrompido e o arquivo parcial é apagado, itens na fila são marcados "Cancelado".
- O cancelamento **DEVE** ter efeito em até 1 segundo **[P]**, exceto dentro de uma única chamada indivisível do núcleo, que verifica o sinal de cancelamento entre etapas.
- Estados de item: Na fila, Executando, Concluído, Falha, Cancelado, Ignorado — sempre com texto, nunca só cor.

#### RF10 — Destino da saída (M · MVP)
**Desktop**
- Padrão: pasta `Morsa` dentro da pasta de Documentos do usuário **[P]**, criada se não existir. Configurável em `SYS-00`.
- Opção por ordem: "Mesma pasta da origem" ou "Escolher pasta…".
- Ao final, botão "Abrir pasta" no resumo.
- Se a pasta de destino não tiver permissão de escrita: `E-201`.

**Web**
- Chromium com File System Access API: o usuário escolhe uma pasta e os arquivos são gravados diretamente nela.
- Demais navegadores: download individual (até 5 arquivos **[P]**) ou um único ZIP com todos os resultados (padrão a partir de 6 **[P]**).
- O ZIP de download é gerado sem compressão (método *stored*), apenas como embalagem.

#### RF11 — Resumo da ordem (M · MVP)
Ao concluir, a ordem exibe: contagem de concluídos, falhas, ignorados e cancelados; formatos de entrada e saída; tempo total; lista de falhas com motivo e sugestão (Apêndice A); botões "Abrir pasta" (desktop), "Baixar" (web), "Copiar relatório" (texto simples para a área de transferência) e "Nova ordem com os mesmos arquivos" **[P]**.

O resumo **NÃO DEVE** destacar quanto os arquivos diminuíram ou cresceram (decisão de design: o app não é compressor). O tamanho de saída de cada item **PODE** aparecer na tabela, como dado neutro **[P]**.

#### RF12 — Expressão de intervalo de páginas (M · MVP)
Formato aceito: números e intervalos separados por vírgula, 1-indexados. Exemplos: `1-3, 5, 8-` (8 até o fim), `-4` (1 a 4), `todas` / `all`. Espaços são ignorados.

**Critérios de aceitação**
- Páginas fora do documento geram erro inline com o total de páginas ("O documento tem 12 páginas").
- Intervalos invertidos (`5-2`) são erro.
- Páginas repetidas são aceitas em `PDF-01` (gera a imagem uma vez) **[P]**.

### 7.3 Ferramentas de PDF

#### RF13 — PDF → imagem (`PDF-01`) (M · MVP)
- Renderiza as páginas escolhidas como PNG, JPG ou WebP, no DPI escolhido.
- Nome de saída: `<nome>-p<NNN>.<ext>`, com número de dígitos conforme o total de páginas (mínimo 2) **[P]**.
- Com mais de uma página, as imagens vão para uma subpasta `<nome>/` dentro do destino **[P]**.
- Formulários e anotações são renderizados como aparecem na tela **[P]**.
- Limite de segurança: página renderizada acima de 200 megapixels **[P]** recebe `E-206` com sugestão de reduzir o DPI.

#### RF14 — Imagens → PDF (`PDF-02`) (M · MVP)
- Uma imagem por página, na ordem da lista, reordenável por arraste ou `Alt+↑/↓`.
- Tamanho da página: "Ajustar à imagem" (padrão **[P]**), A4 ou Carta, com orientação automática por imagem e margem configurável (0, 10, 20 mm **[P]**).
- Imagens JPG são embutidas sem recodificação (sem perda de qualidade); demais formatos são embutidos sem perdas **[P]**.
- Transparência é composta sobre branco **[P]**.
- Saída: `<nome-da-primeira-imagem>.pdf` ou nome definido pelo usuário.

#### RF15 — Juntar PDFs (`PDF-03`) (M · MVP)
- Lista dos PDFs com contagem de páginas; reordenar arquivos por arraste.
- Visualização em grade de miniaturas de todas as páginas, com referência `A1`, `B6` etc.
- Dentro da grade, é possível excluir páginas individuais antes de juntar.
- Marcadores (sumário) dos PDFs de origem são preservados e agrupados sob o nome de cada arquivo **[P]**.
- Links internos são preservados quando possível; formulários com campos de nomes conflitantes são renomeados para não colidir **[P]**.
- Saída: `juntado-AAAAMMDD-HHMMSS.pdf` **[P]** ou nome definido pelo usuário.

#### RF16 — Dividir PDF (`PDF-04`) (M · MVP)
Modos:
1. **Cada página em um arquivo.**
2. **A cada N páginas** (ex.: a cada 2).
3. **Por intervalos** (ex.: `1-3, 4-10, 11-`): cada intervalo vira um arquivo.
4. **Extrair páginas selecionadas** num único arquivo.

- Nome de saída: `<nome>-<NN>.pdf` (modos 1–3) ou `<nome>-extraido.pdf` (modo 4) **[P]**.
- Saídas múltiplas vão para a subpasta `<nome>/` **[P]**.

#### RF17 — Reordenar páginas (`PDF-05`) (M · MVP)
- Grade de miniaturas com seleção múltipla (clique, `Shift+clique`, `Ctrl/Cmd+clique`, `Ctrl/Cmd+A`).
- Operações: mover por arraste (régua azul indica o ponto de inserção), girar 90° (`R`), girar −90° (`Shift+R`), duplicar (`Ctrl/Cmd+D`), remover (`Del`), desfazer (`Ctrl/Cmd+Z`) e refazer (`Ctrl/Cmd+Shift+Z`).
- Também acessível sem mouse: setas movem o foco, `Alt+setas` move a página selecionada.
- Saída: `<nome>-reordenado.pdf` **[P]**.
- Pelo menos uma página precisa permanecer; tentar remover todas é bloqueado com explicação.

#### RF18 — PDFs com senha (M · MVP)
- Ao detectar PDF criptografado, a ordem pede a senha num campo protegido, com a explicação "A senha não é guardada".
- Senha errada: `E-202`, com nova tentativa sem limite.
- Aviso antes de executar: a saída sai sem senha (RN05).
- PDFs com permissões restritas (ex.: proibido imprimir) mas sem senha de abertura **[P]**: o app abre normalmente e exibe aviso informativo; não remove nem altera as permissões nas operações que preservam o documento (`PDF-05`).

### 7.4 Ferramentas de compactados

#### RF19 — Ver conteúdo (`ZIP-01`) (M · MVP)
- Lista em árvore com: nome, tamanho original, data de modificação, e indicadores de pasta, criptografia e "compactado dentro de compactado".
- Totais: número de itens e tamanho total declarado (marcado como "declarado no arquivo").
- Busca por nome dentro da lista.
- Itens com caminhos perigosos (RN09) aparecem marcados com aviso já na listagem.

#### RF20 — Extrair (`ZIP-02`) (M · MVP)
- Extrair tudo ou apenas os itens selecionados na árvore.
- Destino padrão: subpasta com o nome do arquivo (`amostras-fornecedor/`) dentro da pasta de saída **[P]**.
- Preserva a estrutura de pastas e as datas de modificação **[P]**.
- Nomes de arquivo em codificação legada (CP437, comum em ZIPs do Windows) são convertidos para UTF-8; o bit UTF-8 do ZIP é respeitado quando presente.
- Nomes inválidos no sistema de destino (ex.: `con`, `aux`, `nul`, `:` e `?` no Windows) são saneados (Apêndice C) e o saneamento é listado no resumo.
- ZIP criptografado: pede senha (ZipCrypto e AES-256) **[P]**; mesmas regras de RN13.
- Aplica integralmente RN09 e RN10.
- Compactados internos aparecem no resumo com a ação "Extrair este também" (RN10, sem extração automática).
- Permissões Unix e bit executável **NÃO DEVEM** ser aplicados aos arquivos extraídos **[P]** (segurança).

#### RF21 — Criar ZIP (`ZIP-03`) (M · MVP)
- Entrada: arquivos e pastas soltos ou selecionados.
- Estrutura: arquivos soltos ficam na raiz; pastas mantêm a hierarquia relativa.
- Compressão: Deflate em nível padrão **[P]**. Não existe opção de nível de compressão (decisão de escopo: o app não é compressor).
- Nomes gravados em UTF-8 com o bit de linguagem UTF-8 marcado.
- Suporte a ZIP64 para arquivos acima de 4 GB ou mais de 65 535 itens.
- Sem opção de senha no MVP **[P]**.
- Saída: `<nome-da-pasta-ou-primeiro-arquivo>.zip` ou nome definido pelo usuário.

#### RF22 — Formatos compactados adicionais (M · Pós-MVP)
`ZIP-01` e `ZIP-02` passam a aceitar 7z, TAR, GZ, XZ e RAR; `ZIP-03` passa a criar 7z, TAR, TAR.GZ e TAR.XZ. Todas as regras de RF19–RF21, RN09 e RN10 se aplicam. Arquivos GZ ou XZ que contêm um único arquivo (não TAR) são extraídos diretamente.

#### RF23 — Converter compactado (`ZIP-04`) (M · Pós-MVP)
Converte qualquer formato de entrada aceito para ZIP, 7z, TAR, TAR.GZ ou TAR.XZ, passando por extração temporária com todas as proteções e regravação. Arquivos temporários são apagados ao final, inclusive em caso de falha ou cancelamento.

### 7.5 Recursos transversais

#### RF24 — Presets (S · MVP)
- Salvar as opções atuais de uma ferramenta com um nome (ex.: "WebP 80% largura 1920").
- Aplicar, renomear, duplicar e excluir presets.
- Presets são por ferramenta e armazenados localmente; exportar e importar como arquivo JSON **[P]**.
- Presets **NÃO DEVEM** guardar senhas nem caminhos de pasta absolutos de outra máquina **[P]**.

#### RF25 — Histórico (S · MVP)
- Lista das ordens com: número, ferramenta, quantidade de itens, formatos (`TIFF → PNG`), situação e data.
- Filtrar por ferramenta e situação; buscar por nome de arquivo.
- Abrir uma ordem mostra o resumo (RF11) somente leitura.
- "Repetir ordem" reabre a ferramenta com as mesmas opções (os arquivos precisam ser adicionados de novo, pois o histórico não guarda conteúdo).
- Limpar todo o histórico, com confirmação; retenção máxima configurável (padrão: 500 ordens **[P]**, as mais antigas são descartadas).
- Desligar o registro de nomes de arquivos (RN12).

#### RF26 — Busca de ferramentas (S · MVP)
`Ctrl/Cmd+K` abre a busca no índice por código ou nome, com correspondência sem acentos e sem diferenciar maiúsculas ("pdf jun" encontra "PDF-03 Juntar PDFs"). `Enter` abre a ferramenta.

#### RF27 — Configurações (`SYS-00`) (M · MVP)
| Grupo | Configurações |
| --- | --- |
| Geral | Idioma (Automático, Português (Brasil), English); tema (Sistema, Claro, Escuro) |
| Saída | Pasta padrão (desktop); comportamento em conflito de nome (sempre sufixo / perguntar) |
| Imagens | Padrões de qualidade, fundo e metadados |
| Histórico | Registrar nomes de arquivo; retenção; limpar histórico |
| Segurança | Limite total de extração (desktop, 1–100 GB, padrão 10 GB) |
| Sobre | Versão, plataforma, ficha da instalação, licença do Morsa, licenças de terceiros (gerada automaticamente), link do repositório (exibido como texto, abre no navegador somente por ação explícita do usuário) |

Configurações são aplicadas imediatamente, sem botão "Salvar". "Restaurar padrões" por grupo, com confirmação.

#### RF28 — Ficha da instalação (M · MVP)
Bloco exibido no Início e em Sobre, com fatos verificáveis: Versão e plataforma; Rede: **Sem acesso à rede**; Conta: Inexistente; Histórico: Só neste computador (ou "Só neste navegador" na web); Licença: Código aberto (MIT OR Apache-2.0). Não exibe contadores de bytes enviados (decisão de design).

#### RF29 — Integração com o sistema operacional (C · Pós-MVP)
- "Abrir com Morsa" / associação opcional para os formatos de entrada.
- Item "Converter com Morsa" no menu de contexto (Windows Explorer, macOS Finder via extensão de compartilhamento, Linux via arquivos `.desktop` com `MimeType`).
- Abrir o app com arquivos passados por argumento de linha de comando (necessário para as integrações acima). Este item específico é **S · MVP** **[P]**, porque custa pouco e destrava as integrações.

#### RF30 — Idiomas (M · MVP)
- Português (Brasil) e inglês, com detecção automática pelo idioma do sistema e fallback para inglês.
- Todos os textos externalizados em arquivos ARB; nenhum texto fixo no código.
- Números, datas e tamanhos formatados conforme o idioma: pt-BR usa vírgula decimal e espaço fino de milhar (`12 940,1 KB`); inglês usa ponto decimal e vírgula de milhar (`12,940.1 KB`).
- Unidades em base 1024 com símbolos KB, MB, GB **[P]** (convenção mais familiar ao público, documentada no Sobre).
- O layout **DEVE** acomodar textos até 30% mais longos que o português sem cortar informação.

#### RF31 — Primeira execução (M · MVP)
Na primeira execução não há tutorial modal. A tela de Início mostra os três passos ("Soltar arquivos → Confirmar o destino → Converter") no lugar do histórico recente, a pasta de saída padrão e o botão "Abrir arquivos…" como ação primária, já com foco. Os passos somem quando existir a primeira ordem.

---

## 8. Fluxos de uso

### 8.1 Fluxo principal: converter imagens (3 cliques)

1. Usuário solta 12 arquivos TIFF na janela. **(arrasto, não conta como clique)**
2. Todos são imagens: a triagem é pulada e `IMG-01` abre com os 12 itens, destino proposto PNG. **Clique 1:** escolhe "JPG" no seletor de destino.
3. Pasta de saída já preenchida com o padrão.
4. **Clique 2:** "Converter" (ou `Ctrl/Cmd+Enter`).
5. Resumo: "12 concluídos". **Clique 3 (opcional):** "Abrir pasta".

Meta: uma conversão simples não exige mais de 3 cliques após soltar os arquivos (RNF06).

### 8.2 Fluxo alternativo: tipos misturados

1. Usuário solta 5 imagens, 2 PDFs, 1 ZIP e 1 XLSX.
2. Triagem: 3 grupos (`IMG-01`, `PDF-03`, `ZIP-02`) e 1 ignorado (XLSX, com motivo).
3. Usuário troca a ferramenta do grupo de PDF para `PDF-05` no painel lateral (opcional).
4. "Abrir 3 ordens": as três abrem como rascunhos.
5. Cada ordem é configurada e executada independentemente.

### 8.3 Fluxo de exceção: falha parcial no lote

1. Lote de 8 imagens; o item 005 está corrompido.
2. Itens 001–004 e 006–008 concluem; 005 recebe "Falha".
3. Resumo: "7 concluídos · 1 falha", com a linha "005 · scan-03.bmp terminou antes do esperado. Tente reexportar o arquivo na origem." (`E-105`).
4. O histórico registra a ordem como "1 falha".

### 8.4 Fluxo de exceção: zip bomb

1. Usuário solta `amostras.zip` (40 KB) e escolhe `ZIP-02`.
2. Durante a extração, um item ultrapassa a taxa de 1000:1.
3. A extração pausa e pergunta: "O item `dados.bin` expande mais de 1000 vezes. Isso é incomum e pode indicar um arquivo malicioso. Continuar mesmo assim?" com "Cancelar" (padrão, com foco) e "Continuar".
4. Se o usuário continuar e o total ultrapassar 10 GB, a extração para definitivamente, o conteúdo parcial é apagado e o resumo mostra `E-302`.

### 8.5 Fluxo de exceção: PDF com senha

1. Usuário abre `contrato.pdf` em `PDF-03` junto com outros dois PDFs.
2. O item mostra "Protegido por senha" e um campo de senha.
3. Usuário informa a senha; o app valida na hora.
4. Antes de juntar: aviso "O PDF resultante não terá senha."
5. Execução normal.

### 8.6 Fluxo de exceção: disco cheio

1. Durante uma conversão, o espaço livre cai abaixo do necessário.
2. O item em andamento falha com `E-203`, o arquivo parcial é apagado, e os itens restantes são marcados como "Cancelado" com o mesmo motivo **[P]** (não adianta tentar os outros).

---

## 9. Interface e design

O sistema visual completo está documentado em `design/Sistema Visual.dc.html` (DOC-01). Esta seção consolida as regras normativas e registra os ajustes decididos depois dos protótipos, que **prevalecem** sobre os arquivos de design.

### 9.1 Conceito

Estética de **bancada técnica / catálogo impresso**: tinta sobre papel, réguas finas, tabelas densas, cantos retos, sem gradientes, sem sombras internas, sem cor de marca. A cor aparece apenas em etiquetas de categoria e estados.

### 9.2 Ajustes posteriores aos protótipos (normativos)

| # | Ajuste | Motivo |
| --- | --- | --- |
| A1 | Vocabulário comum: "Ferramenta" (não "peça"), "Histórico" (não "livro de ordens"), botão com o verbo da ação ("Converter", "Juntar", "Dividir", "Reordenar", "Extrair", "Criar ZIP"). O número da ordem (`OS-0143`) aparece só como detalhe secundário, nunca como título de tela. | Público não técnico não deve aprender uma metáfora para converter um PNG. |
| A2 | Remover o contador "0 B enviados" da ficha e da barra de status. Usar a afirmação fixa "Sem acesso à rede". | O app não mede tráfego; um número que nunca muda é teatro. |
| A3 | Listas e histórico mostram formatos (`TIFF → PNG`) e quantidade de arquivos, nunca tamanho antes → depois. | O app não é compressor. |
| A4 | Barra de título nativa de cada sistema operacional, não desenhada pelo app. | Botões de janela diferem por sistema. |
| A5 | Atalhos exibidos com `Cmd` no macOS e `Ctrl` no Windows e Linux. | Convenção de cada plataforma. |
| A6 | Tipografia 2b: IBM Plex Sans Condensed nos títulos e rótulos de seção; Ioskeley Mono **NL** no restante. A Univers foi descartada (licença comercial). | Hierarquia visual com licença OFL. |
| A7 | Layout responsivo para janelas estreitas (seção 9.8). | Requisito da versão web. |
| A8 | Na web, a sobreposição de arraste não mostra contagem por tipo (o navegador não expõe nomes antes de soltar); mostra apenas "Solte para começar". | Limitação da plataforma. |
| A9 | Textos: "Executar ordem" → verbo da ferramenta; "Ordens recentes" → "Histórico recente"; "Sem peça" → "Sem ferramenta"; "Peças propostas" → "Ferramentas propostas"; "Nova ordem" (título do Início) → "Início". | Consequência de A1. |

### 9.3 Tipografia

| Papel | Fonte | Tamanho / peso | Observação |
| --- | --- | --- | --- |
| Título de tela | IBM Plex Sans Condensed | 28–32 px / 700 | `letter-spacing: -0,01em` |
| Rótulo de seção | IBM Plex Sans Condensed | 11–12 px / 700, caixa alta | `letter-spacing: 0,06–0,08em` |
| Título de painel | Ioskeley Mono NL | 16 px / 700 | |
| Corpo, células, campos | Ioskeley Mono NL | 13 px / 400 | |
| Ênfase, botões, valores | Ioskeley Mono NL | 13 px / 500 | |
| Ficha e tabela | Ioskeley Mono NL | 12 px / 400 | |
| Metadado | Ioskeley Mono NL | 11 px / 300 | Light só a partir de 11 px |

- Fontes empacotadas no app (arquivos `.ttf` em `app/assets/fonts/`), com os arquivos `OFL.txt` de cada família.
- Variantes Ioskeley: **Hinted** para Windows e Linux em telas de densidade padrão, **Unhinted** para macOS e telas HiDPI **[P]**; na web, o pacote `IoskeleyMono-Web` (subconjunto Latin), variante sem ligaduras.
- Nenhuma fonte carregada da internet (RNF10). Fallback: `ui-monospace, monospace`.
- Números sempre alinhados à direita, com a unidade na mesma célula.

### 9.4 Cores (tokens)

| Token | Claro | Escuro | Uso |
| --- | --- | --- | --- |
| `--paper` | `#fafaf8` | `#121211` | Fundo |
| `--ink` | `#121212` | `#e8e8e4` | Texto e régua forte (≈18:1) |
| `--ink2` | `#5d5d58` | `#a2a29c` | Texto secundário (≈6,5:1) |
| `--weak` | `#dcdcd7` | `#3d3d3a` | Régua fraca |
| `--weak2` | `#cfcfca` | `#4a4a46` | Régua fraca 2, pauta, borda de atalho |
| `--ctrl` | `#121212` | `#6a6a65` | Borda de controle (≥3:1) |
| `--sunk` | `#f0f0ec` | `#1c1c1a` | Hover |
| `--sel` | `#e6e6e1` | `#262624` | Linha selecionada |
| `--inkH` | `#3a3a37` | `#ffffff` | Botão primário em hover |
| `--inkP` | `#000000` | `#c9c9c4` | Botão primário pressionado |
| `--dis` | `#a3a39e` | `#5f5f5a` | Desabilitado |
| `--ok` | `oklch(0.45 0.12 150)` | `oklch(0.74 0.13 150)` | Concluído |
| `--run` | `oklch(0.45 0.15 255)` | `oklch(0.74 0.12 255)` | Executando |
| `--fail` | `oklch(0.48 0.18 28)` | `oklch(0.72 0.15 28)` | Falha |
| `--focus` | `oklch(0.50 0.16 255)` | `oklch(0.72 0.14 255)` | Anel de foco |
| `--img` | `oklch(0.62 0.13 235)` | idem | Etiqueta Imagens |
| `--pdf` | `oklch(0.60 0.16 30)` | idem | Etiqueta PDF |
| `--zip` | `oklch(0.74 0.13 85)` | idem | Etiqueta Compactados |
| `--fut` | `#8f8f8a` | idem | Etiqueta Em breve |

- Tokens implementados como `ThemeExtension` do Flutter, com valores OKLCH convertidos para sRGB em tempo de build **[P]**.
- Etiquetas de categoria aparecem só como quadrados sólidos de 8–12 px ao lado do código; o texto ao lado é sempre `--ink`.
- Estados usam cor também no texto (todas as variantes passam 4,5:1 sobre o papel).
- Hover é tom de papel, nunca cor.

### 9.5 Grade e medidas

- Base de espaçamento 4 px; escala 4, 8, 12, 16, 24, 32.
- Altura de linha de tabela: 28 px; linha de índice: 24 px; linha de ficha: 26 px.
- Controles (botões, campos, seletores): 28 px de altura, cantos retos (`border-radius: 0`).
- Réguas: forte 2 px (sob cabeçalhos de seção e acima de totais); de tabela 1 px `--ink`; fraca 1 px `--weak` entre linhas; tracejada 1 px só em áreas de soltar.
- Pauta de 24 px como fundo de áreas de soltar e miniaturas.
- Estrutura da janela (desktop, ≥1024 px): índice 250 px | conteúdo flexível | painel de parâmetros 280 px; barra de status 24 px.
- Janela padrão 1200 × 760; mínimo 1024 × 640 **[P]** no desktop.

### 9.6 Telas

| ID | Tela | Conteúdo principal |
| --- | --- | --- |
| T01 | Início (uso recorrente) | Área de soltar, histórico recente (6 últimas), ficha da instalação, atalhos |
| T02 | Início (primeira execução) | Área de soltar ampliada, 3 passos, ficha, pasta de saída padrão |
| T03 | Sobreposição de arraste | Borda tracejada 2 px e pauta cobrindo o conteúdo; contagem por tipo só no desktop |
| T04 | Triagem | Grupos com ferramenta proposta, destino, inclusão; painel de ferramentas compatíveis |
| T05 | `IMG-01` Converter imagens | Tabela de itens, destino, opções, edição, pré-visualização, progresso, resumo |
| T06 | `PDF-01` PDF → imagem | Lista de PDFs, intervalo, DPI, formato |
| T07 | `PDF-02` Imagens → PDF | Lista ordenável, tamanho de página, margem |
| T08 | `PDF-03` Juntar | Lista de PDFs + grade de miniaturas |
| T09 | `PDF-04` Dividir | Modo de divisão, intervalos, pré-visualização dos arquivos resultantes |
| T10 | `PDF-05` Reordenar | Grade de miniaturas editável |
| T11 | `ZIP-01` Ver conteúdo | Árvore de itens, totais, avisos |
| T12 | `ZIP-02` Extrair | Árvore com seleção, destino, aviso de limites |
| T13 | `ZIP-03` Criar ZIP | Lista de arquivos e pastas, nome de saída |
| T14 | Histórico | Tabela filtrável, detalhe da ordem |
| T15 | `SYS-00` Configurações | Grupos da RF27 |
| T16 | Diálogos | Senha, conflito de nome, confirmação de taxa de compressão, limpar histórico |

Cada tela de ferramenta segue a mesma estrutura: cabeçalho (caminho "Início · Ferramenta", título com código e nome, número da ordem e data à direita) → corpo (tabela ou grade) → barra inferior com totais e ações (secundária à esquerda da primária) → painel lateral de parâmetros.

### 9.7 Componentes

Botões (primário, secundário, link, com atalho), campos (normal, foco, erro, desabilitado), seletor, controle segmentado, caixa de seleção (marcado, desmarcado, parcial), menu de contexto, linha de tabela (normal, hover, selecionada, foco), ficha rótulo → valor, barras de progresso (item e ordem), avisos (atenção, falha), área de soltar, miniaturas de página, barra de status. Estados e medidas conforme DOC-01 §5.

Regras:
- Atalho sempre visível no próprio controle (11 px, borda `--weak2`).
- Foco: anel azul de 2 px com respiro de 2 px por fora em controles, e por dentro (−2 px) em linhas de tabela, para não ser cortado pela rolagem.
- Seleção de linha: tom `--sel` + caixa marcada. A ferramenta ativa no índice é a única inversão total (fundo `--ink`, texto `--paper`).
- Desabilitado é isento de contraste (WCAG 1.4.3), mas o texto permanece legível.

### 9.8 Layout responsivo [P]

| Largura da janela | Índice | Painel de parâmetros | Observação |
| --- | --- | --- | --- |
| ≥ 1280 px | Fixo, 250 px | Fixo, 280 px | Layout completo |
| 1024–1279 px | Fixo, 250 px | Fixo, 280 px | Colunas de tabela secundárias (data, tamanho) podem ser ocultadas |
| 720–1023 px | Recolhido em trilho de 48 px com códigos de categoria; expande como gaveta sobreposta | Vira seção abaixo do conteúdo ou gaveta à direita | Só web; desktop tem mínimo de 1024 px |
| < 720 px | Menu em gaveta acionado por botão no cabeçalho | Seção empilhada abaixo do conteúdo | Tabelas viram listas de duas linhas por item; grade de miniaturas com 2–3 colunas |

### 9.9 Barra de status

Fixa no rodapé: `● OFFLINE · Sem acesso à rede · Processamento local` à esquerda; `Morsa · v<versão>` à direita. Durante uma ordem, o trecho central mostra o progresso resumido ("Convertendo · 3 de 12").

### 9.10 Ícone

Ícone escolhido pelo usuário, fornecido em `assets/icone.svg`. Derivados gerados no build **[P]**: Windows `.ico` (16, 24, 32, 48, 64, 128, 256), macOS `.icns` (16 a 1024, @1x e @2x), Linux PNG (16 a 512, hicolor), PWA (192, 512 e versão *maskable* com margem de segurança de 10%), favicon.

---

## 10. Requisitos não funcionais

### RNF01 — Plataformas (M)
| Plataforma | Versões mínimas | Arquiteturas |
| --- | --- | --- |
| Windows | 10 (versão 1809) ou superior **[P]** | x64; ARM64 **(S)** |
| macOS | 12 Monterey ou superior | Apple Silicon (arm64) e Intel (x64), binário universal |
| Linux | Ubuntu 22.04+, Fedora 38+, Debian 12+ e derivados **[P]** | x64; ARM64 **(C)** |
| Web | Duas últimas versões estáveis de Chrome, Edge, Firefox e Safari | — |

A web exige WebAssembly, Web Workers e Service Worker. Navegadores sem suporte veem uma página estática explicando o requisito.

### RNF02 — Desempenho (M)
Medido em máquina de referência **[P]**: 4 núcleos, 8 GB de RAM, SSD.

| Operação | Meta (desktop) | Meta (web) |
| --- | --- | --- |
| Abrir o app até a tela de Início interativa | ≤ 2 s | ≤ 3 s após cache |
| Detectar formato de 100 arquivos | ≤ 500 ms | ≤ 1 s |
| Converter imagem de 10 MB (ex.: TIFF 4000×3000 → PNG) | ≤ 2 s | ≤ 6 s |
| Converter 100 JPGs de 4 MB para WebP q85 | ≤ 60 s | ≤ 4 min |
| Renderizar 1 página A4 de PDF a 150 DPI | ≤ 300 ms | ≤ 1 s |
| Juntar 10 PDFs com 100 páginas no total | ≤ 3 s | ≤ 8 s |
| Extrair ZIP de 1 GB com 1 000 itens | ≤ 30 s (limitado pelo disco) | ≤ 90 s |
| Gerar grade de 100 miniaturas de PDF | ≤ 5 s, progressivo | ≤ 15 s, progressivo |

- A interface **NUNCA DEVE** travar: nenhum quadro acima de 100 ms na thread de interface durante processamento **[P]**.
- Todo processamento roda fora da thread de interface (isolates/threads nativas no desktop, Web Worker na web).

### RNF03 — Uso de recursos (M) [P]
- Memória ociosa: ≤ 200 MB (desktop).
- O processamento de imagens grandes **DEVE** operar por streaming ou com limite: decodificações acima de 500 megapixels são recusadas (SEG06).
- Lote de qualquer tamanho não pode manter mais de N imagens decodificadas em memória ao mesmo tempo, onde N = número de workers.
- Arquivos temporários ficam na pasta temporária do sistema, numa subpasta `morsa-<id-da-sessão>`, e são apagados ao fim de cada ordem e ao iniciar o app (limpeza de sessões anteriores interrompidas).

### RNF04 — Offline (M)
- Desktop: funciona 100% sem internet, desde a instalação.
- Web: após o primeiro carregamento, funciona 100% offline (Service Worker com cache de todos os recursos). O app **DEVE** indicar quando está pronto para uso offline **[P]**.

### RNF05 — Privacidade (M)
- Nenhuma telemetria, análise de uso, relatório de falhas automático, identificador de instalação ou verificação de versão pela rede.
- Logs de diagnóstico existem apenas localmente, são desligados por padrão **[P]**, não contêm conteúdo de arquivos nem senhas, e só são compartilhados se o usuário copiar e enviar por conta própria.

### RNF06 — Usabilidade (M)
- Conversão simples em no máximo 3 cliques após soltar os arquivos (8.1).
- Toda ação disponível por mouse também disponível por teclado (Apêndice B).
- Mensagens de erro em linguagem simples, com o que aconteceu e o que fazer (Apêndice A); nunca códigos técnicos isolados ou stack traces na interface.
- Nenhuma ação destrutiva sem confirmação (limpar histórico, excluir preset, substituir arquivo).

### RNF07 — Acessibilidade (M)
Conformidade com **WCAG 2.1 nível AA**:
- Contraste mínimo de 4,5:1 para texto e 3:1 para componentes de interface e bordas de controle, em ambos os temas.
- Navegação completa por teclado, ordem de foco lógica, foco sempre visível (anel `--focus`).
- Semântica para leitores de tela (widgets `Semantics` do Flutter): rótulos em todos os controles, anúncio de mudanças de progresso e de conclusão (região *live*), estados de item anunciados como texto.
- Testado com NVDA (Windows), VoiceOver (macOS) e Orca (Linux) **[P]** antes de cada versão.
- Respeita a preferência do sistema por movimento reduzido (sem animações além de transições ≤ 150 ms).
- Escala de texto do sistema até 200% sem perda de função.
- Alvos de clique com no mínimo 24 × 24 px (WCAG 2.2 2.5.8, adotado por antecipação **[P]**).

### RNF08 — Internacionalização (M)
Ver RF30. Adicionar um idioma novo não exige mudança de código, apenas um arquivo ARB.

### RNF09 — Manutenibilidade (M)
- Cada ferramenta é um módulo independente em `app/lib/tools/<codigo>/` e, se precisar de processamento, em `core/src/<categoria>/`.
- Adicionar uma ferramenta não exige alterar o núcleo de outras ferramentas, apenas registrá-la no catálogo.
- Cobertura de testes do núcleo Rust ≥ 80% de linhas **[P]**.
- Código formatado e verificado automaticamente: `cargo fmt`, `cargo clippy -D warnings`, `dart format`, `flutter analyze` sem avisos.

### RNF10 — Zero rede (M)
- O app **NÃO DEVE** fazer nenhuma requisição de rede em tempo de execução.
- Desktop: o binário não deve incluir clientes HTTP; a CI verifica a ausência de dependências como `reqwest`, `hyper`, `ureq` e do pacote `http` do Dart **[P]**.
- Web: Content Security Policy restritiva (`connect-src 'self'`, `default-src 'self'`, sem domínios externos); build com `--no-web-resources-cdn`; teste automatizado com navegador sem rede verifica que nenhuma requisição sai da origem após o carregamento.
- Abrir links externos (ex.: repositório no Sobre) só acontece por clique explícito e é delegado ao navegador do sistema, não feito pelo app.

### RNF11 — Código aberto (M)
Ver seção 17.

### RNF12 — Confiabilidade (M)
- Falha do núcleo ao processar um arquivo (incluindo *panic* em Rust) **NÃO DEVE** derrubar o app: o erro é capturado na fronteira da ponte e vira falha do item (`E-199`).
- Escrita de saída atômica: grava em arquivo temporário na mesma pasta de destino e renomeia ao final **[P]**; nunca deixa arquivo parcial com o nome final.
- Fechar o app durante uma ordem pede confirmação; se confirmado, itens em andamento são cancelados e temporários apagados.

### RNF13 — Tamanho do app [P]
| Pacote | Meta |
| --- | --- |
| Instalador Windows | ≤ 60 MB |
| DMG macOS (universal) | ≤ 90 MB |
| AppImage Linux | ≤ 70 MB |
| Web (download inicial, comprimido) | ≤ 15 MB |

---

## 11. Segurança e privacidade

### 11.1 Modelo de ameaça

O principal vetor é o **arquivo malicioso ou malformado** que o usuário recebeu de terceiros e abre no app. Objetivos do atacante: executar código, escrever fora da pasta de destino, esgotar disco ou memória, ou travar o app. Não há servidor, conta ou tráfego de rede a proteger.

### 11.2 Requisitos de segurança

| ID | Requisito |
| --- | --- |
| SEG01 | Todo parsing de arquivo acontece no núcleo Rust, em código seguro em memória. Bibliotecas em C (PDFium; libarchive no Pós-MVP) são aceitas por necessidade e **DEVEM** ser mantidas atualizadas e testadas com *fuzzing*. |
| SEG02 | Código `unsafe` em Rust escrito pelo projeto é proibido fora das camadas de ligação com bibliotecas C, e cada bloco `unsafe` tem comentário justificando a segurança. |
| SEG03 | Proteção contra zip slip (RN09): normalizar cada caminho, rejeitar absolutos, `..`, unidades e UNC, e confirmar que o caminho final resolvido está dentro da pasta de destino. Links simbólicos dentro do pacote não são criados no MVP **[P]**; são extraídos como itens ignorados com aviso. |
| SEG04 | Proteção contra zip bomb (RN10): contadores de bytes extraídos por item e total, verificados durante a escrita; taxa de compressão calculada sobre os bytes reais. |
| SEG05 | Compactados aninhados nunca são abertos automaticamente. |
| SEG06 | Proteção contra decompression bomb em imagens: ler dimensões do cabeçalho antes de decodificar e recusar imagens acima de 500 megapixels **[P]** ou 32 768 px em qualquer lado **[P]** (`E-106`). Limite de alocação por decodificação: 4 GB no desktop, 1 GB na web **[P]**. |
| SEG07 | Timeout por operação de item: 5 minutos **[P]**, com mensagem `E-107`. Na web, um worker que não responde é encerrado e recriado. |
| SEG08 | SVG: sem execução de scripts, sem busca de recursos externos, sem entidades XML externas (XXE), limite de profundidade de aninhamento e de quantidade de elementos **[P]**. |
| SEG09 | PDF: JavaScript embutido nunca é executado; ações de abrir URL ou arquivo são ignoradas; anexos não são extraídos no MVP. |
| SEG10 | Arquivos extraídos não recebem permissão de execução (RF20). |
| SEG11 | Senhas: só em memória, zeradas após uso quando possível (`zeroize`), nunca logadas (RN13). |
| SEG12 | Dependências auditadas a cada build com `cargo audit` e `cargo deny check advisories`; vulnerabilidade conhecida em dependência bloqueia a versão. |
| SEG13 | Web: CSP restritiva (RNF10), sem `eval` (exceção registrada em DC13), sem conteúdo de terceiros, servida com `Cross-Origin-Opener-Policy` e `Cross-Origin-Embedder-Policy` quando necessário para os workers **[P]**. |
| SEG14 | *Fuzzing* contínuo (`cargo-fuzz`) para os decodificadores de entrada: detecção de formato, imagens, SVG, PDF e ZIP. Cada falha encontrada vira caso no corpus de regressão. |
| SEG15 | Política de divulgação responsável publicada em `SECURITY.md`, com canal privado de contato. |

### 11.3 Privacidade

- Dados persistidos localmente: configurações, presets, histórico (RN12) e contador de ordens. Nada mais.
- Localização: pasta de dados de aplicativo do sistema (ex.: `%APPDATA%\Morsa`, `~/Library/Application Support/Morsa`, `~/.local/share/morsa`) **[P]**; na web, IndexedDB da origem.
- Botão "Apagar todos os dados do Morsa" em Configurações, que remove configurações, presets e histórico **[P]**.
- Conformidade: como não há coleta, tratamento ou transferência de dados pessoais pelo projeto, a LGPD e a GDPR não impõem obrigações operacionais além de manter essa garantia verdadeira e documentada.

---

## 12. Arquitetura técnica

### 12.1 Visão geral

```
┌──────────────────────────────────────────────────────────┐
│ Flutter (app/) — interface                                │
│  telas · navegação · temas · i18n · estado · persistência │
└───────────────┬──────────────────────────────────────────┘
                │ flutter_rust_bridge (código gerado)
┌───────────────▼──────────────────────────────────────────┐
│ Núcleo Rust (core/) — todo o processamento                │
│  detect · image · svg · pdf · archive · api               │
│  recebe bytes + opções → devolve bytes + relatório        │
└───────┬───────────────────────────────┬──────────────────┘
        │ compila para nativo           │ compila para WASM
┌───────▼─────────┐             ┌───────▼──────────────────┐
│ Desktop          │             │ Web (PWA)                 │
│ Win · macOS · Lx │             │ Web Worker · Service      │
│ offline          │             │ Worker · offline após 1º  │
└──────────────────┘             └──────────────────────────┘
```

### 12.2 Princípios

1. **A interface nunca processa arquivos.** Ela chama o núcleo e exibe o resultado.
2. **O núcleo não conhece telas nem rede.** Recebe dados e opções, devolve dados, relatório e erros tipados.
3. **Uma implementação, duas plataformas.** O mesmo código Rust compila para nativo e para WASM; diferenças de plataforma ficam isoladas em módulos `platform/` **[P]**.
4. **Streaming quando possível.** No desktop, o núcleo pode ler e escrever por caminho de arquivo para evitar carregar arquivos grandes inteiros na memória; na web, opera sobre `Uint8Array`/`Blob` **[P]**.

### 12.3 Estrutura do repositório

```
morsa/
├── app/                        # Flutter
│   ├── lib/
│   │   ├── main.dart
│   │   ├── app/                # roteamento, tema, i18n, atalhos
│   │   ├── design/             # tokens, componentes do sistema visual
│   │   ├── catalog/            # registro de ferramentas e categorias
│   │   ├── tools/
│   │   │   ├── img_01/         # Converter imagens
│   │   │   ├── pdf_01/ … pdf_05/
│   │   │   └── zip_01/ … zip_03/
│   │   ├── triage/             # tela de triagem
│   │   ├── history/            # histórico
│   │   ├── settings/           # SYS-00
│   │   ├── jobs/               # fila de ordens, workers, cancelamento
│   │   ├── storage/            # persistência local
│   │   └── core_api/           # código gerado pela ponte
│   ├── assets/{fonts,icons}/
│   ├── l10n/                   # app_pt.arb, app_en.arb
│   ├── web/                    # index.html, manifest, service worker, CSP
│   └── test/ · integration_test/
├── core/                       # Rust
│   ├── Cargo.toml
│   ├── src/
│   │   ├── lib.rs
│   │   ├── api.rs              # funções expostas à ponte
│   │   ├── detect.rs           # detecção por magic bytes
│   │   ├── error.rs            # erros tipados ↔ códigos E-xxx
│   │   ├── limits.rs           # limites de segurança centralizados
│   │   ├── progress.rs         # progresso e cancelamento
│   │   ├── image/              # decodificar, transformar, codificar
│   │   ├── svg/
│   │   ├── pdf/                # render (PDFium), manipular (lopdf)
│   │   └── archive/            # zip (MVP); 7z, tar, rar (Pós-MVP)
│   ├── tests/                  # testes de integração com o corpus
│   ├── fuzz/                   # alvos cargo-fuzz
│   └── benches/
├── corpus/                     # arquivos de teste (válidos e problemáticos)
├── docs/                       # este documento, arquitetura, como compilar
├── design/                     # arquivos do Claude Design
├── scripts/                    # geração de ícones, licenças de terceiros
├── .github/workflows/
├── deny.toml                   # política de licenças e auditoria
├── LICENSE-MIT · LICENSE-APACHE
├── README.md · CONTRIBUTING.md · SECURITY.md · CODE_OF_CONDUCT.md
└── CHANGELOG.md
```

### 12.4 Contrato da API do núcleo [P]

Todas as funções são assíncronas do ponto de vista do Flutter e reportam progresso por stream.

```rust
// Detecção
fn detect(header: Vec<u8>, file_name: String) -> Detection;

// Imagens
fn image_probe(input: Input) -> Result<ImageInfo, CoreError>;            // dimensões, formato, quadros, alfa
fn image_convert(input: Input, ops: ImageOps, target: ImageTarget,
                 out: Output, ctl: JobControl) -> Result<ItemReport, CoreError>;
fn image_thumbnail(input: Input, max_side: u32) -> Result<Vec<u8>, CoreError>;

// PDF
fn pdf_probe(input: Input, password: Option<String>) -> Result<PdfInfo, CoreError>;
fn pdf_render(input: Input, pages: PageSet, dpi: u32, target: ImageTarget,
              out: OutputDir, ctl: JobControl) -> Result<ItemReport, CoreError>;
fn pdf_from_images(inputs: Vec<Input>, layout: PageLayout,
                   out: Output, ctl: JobControl) -> Result<ItemReport, CoreError>;
fn pdf_merge(inputs: Vec<PdfSource>, plan: Vec<PageRef>,
             out: Output, ctl: JobControl) -> Result<ItemReport, CoreError>;
fn pdf_split(input: PdfSource, mode: SplitMode,
             out: OutputDir, ctl: JobControl) -> Result<ItemReport, CoreError>;
fn pdf_rearrange(input: PdfSource, plan: Vec<PageEdit>,
                 out: Output, ctl: JobControl) -> Result<ItemReport, CoreError>;

// Compactados
fn archive_list(input: Input, password: Option<String>) -> Result<ArchiveListing, CoreError>;
fn archive_extract(input: Input, selection: Selection, dest: OutputDir,
                   limits: ExtractLimits, ctl: JobControl) -> Result<ExtractReport, CoreError>;
fn archive_create(entries: Vec<EntrySource>, format: ArchiveFormat,
                  out: Output, ctl: JobControl) -> Result<ItemReport, CoreError>;
```

- `Input`/`Output`: caminho de arquivo (desktop) ou bytes/handle (web).
- `JobControl`: sinal de cancelamento, canal de progresso, canal de perguntas ao usuário (ex.: confirmação de taxa de compressão).
- `CoreError`: enum com código estável `E-xxx`, mensagem técnica para log e parâmetros para a mensagem localizada na interface.
- Toda função de API envolve o processamento com captura de *panic* (RNF12).

### 12.5 Concorrência

- Desktop: a fila de ordens (`jobs/`) despacha itens para o núcleo, que executa em um pool de threads (`rayon`) com tamanho conforme RF04.
- Web: o núcleo WASM roda num Web Worker dedicado; a interface se comunica por mensagens. Paralelismo com múltiplos workers é **(C)** pós-MVP.
- Cancelamento cooperativo: o núcleo verifica `JobControl` entre etapas (por página, por item de ZIP, por linha de imagem em operações longas).

### 12.6 Persistência local

| Dado | Desktop | Web |
| --- | --- | --- |
| Configurações | JSON em pasta de dados do app | IndexedDB |
| Presets | JSON | IndexedDB |
| Histórico e contador de ordens | SQLite (`sqflite_common_ffi` ou `drift`) **[P]** | IndexedDB |

Esquema versionado com migrações; dados corrompidos são renomeados para `.bak` e o app reinicia com padrões, avisando o usuário **[P]**.

### 12.7 Estado e navegação [P]

- Gerenciamento de estado: Riverpod.
- Navegação: `go_router`, com rotas por código de ferramenta (`/tool/img-01`).
- Ordens abertas convivem como abas ou lista lateral de rascunhos (decisão de design pendente, D7).

---

## 13. Dependências e licenciamento

### 13.1 Política

- Licença do projeto: **MIT OR Apache-2.0**.
- Licenças aceitas em dependências: MIT, Apache-2.0, BSD-2-Clause, BSD-3-Clause, Zlib, ISC, Unicode-DFS/Unicode-3.0, BSL-1.0, MPL-2.0 **somente** para fontes ou dados não linkados **[P]**, OFL-1.1 para fontes.
- **Proibidas:** GPL, AGPL, LGPL (todas as versões) e licenças proprietárias ou "source available".
- Motivo da exclusão da LGPL: na web, todo o núcleo vira um único WASM, o que equivale a linkagem estática.
- Verificação automática: `cargo deny check licenses` (Rust) e verificação equivalente para pacotes Dart **[P]** (`flutter pub deps` + lista de licenças) na CI. Falha bloqueia o merge.
- Arquivo de licenças de terceiros gerado automaticamente a cada build e exibido em Configurações › Sobre.

### 13.2 Dependências previstas

| Uso | Biblioteca | Linguagem | Licença | Fase |
| --- | --- | --- | --- | --- |
| Ponte | `flutter_rust_bridge` | Rust/Dart | MIT | MVP |
| Imagens (decod./codif.) | `image` | Rust | MIT / Apache-2.0 | MVP |
| WebP com perdas | `webp` / `libwebp` | Rust/C | MIT / BSD | MVP |
| Reamostragem rápida | `fast_image_resize` **[P]** | Rust | MIT / Apache-2.0 | MVP |
| Quantização para GIF | `color_quant` / `imagequant` (verificar licença) | Rust | MIT (color_quant) | MVP |
| SVG | `resvg`, `usvg` | Rust | MIT / Apache-2.0 | MVP |
| Renderizar PDF | PDFium via `pdfium-render` | C++/Rust | BSD-3 / Apache-2.0 | MVP |
| Manipular PDF | `lopdf` | Rust | MIT | MVP |
| Gerar PDF a partir de imagens | `lopdf` ou `printpdf` **[P]** | Rust | MIT | MVP |
| ZIP | `zip` | Rust | MIT | MVP |
| Paralelismo | `rayon` | Rust | MIT / Apache-2.0 | MVP |
| Zerar senhas | `zeroize` | Rust | MIT / Apache-2.0 | MVP |
| 7z | `sevenz-rust` | Rust | Apache-2.0 | Pós-MVP |
| TAR, GZ, XZ | `tar`, `flate2`, `lzma-rs` | Rust | MIT / Apache-2.0 | Pós-MVP |
| RAR (extração) | libarchive | C | BSD-2 | Pós-MVP |
| Estado (Flutter) | `flutter_riverpod` | Dart | MIT | MVP |
| Navegação | `go_router` | Dart | BSD-3 | MVP |
| Arrastar e soltar (desktop) | `desktop_drop` | Dart | MIT/Apache **[P]** | MVP |
| Seletor de arquivos | `file_selector` | Dart | BSD-3 | MVP |
| Persistência | `drift` ou `sqflite_common_ffi` | Dart | MIT | MVP |
| Fontes | IBM Plex Sans Condensed, Ioskeley Mono NL | — | OFL-1.1 | MVP |

Qualquer dependência nova passa pela verificação de licença antes do merge. Bibliotecas marcadas com "verificar" **DEVEM** ter a licença confirmada no primeiro uso.

### 13.3 PDFium

- Distribuído como biblioteca dinâmica pré-compilada por plataforma (fonte: builds públicos BSD/Apache, ex.: `pdfium-binaries`) **[P]**, com versão fixada e checksum verificado no build.
- Na web, compilado para WASM (o projeto `pdfium-binaries` publica build WASM) **[P]**.
- Atualizado a cada nova versão estável com correções de segurança.

### 13.4 Formatos excluídos por licença ou patente

| Formato | Motivo |
| --- | --- |
| HEIC/HEIF | libheif é LGPL; codec HEVC tem patentes ativas |
| Criação de RAR | Formato proprietário; algoritmo de compressão não licenciado; o código "unrar" oficial tem licença restritiva |

---

## 14. Qualidade e testes

### 14.1 Estratégia

| Nível | Ferramenta | Escopo |
| --- | --- | --- |
| Unitário (núcleo) | `cargo test` | Detecção, transformações, regras de nome, validações, limites |
| Integração (núcleo) | `cargo test` + corpus | Conversões reais ponta a ponta para cada par da matriz |
| Fuzzing | `cargo-fuzz` | Decodificadores de entrada (SEG14) |
| Propriedade | `proptest` **[P]** | Expressões de intervalo, saneamento de nomes, caminhos de extração |
| Widget (Flutter) | `flutter test` | Componentes e telas isoladas |
| Integração (app) | `integration_test` | Fluxos completos por plataforma |
| Golden (visual) | testes golden do Flutter | Telas principais nos dois temas e dois idiomas |
| Zero rede | teste automatizado (TST10) | Desktop e web |
| Acessibilidade | `flutter test` com verificações de semântica + testes manuais | RNF07 |
| Desempenho | `criterion` (benchmarks) | Metas da RNF02, com alerta de regressão > 20% **[P]** |

### 14.2 Requisitos de teste

| ID | Requisito |
| --- | --- |
| TST01 | Todo par de conversão da matriz (5.2) tem ao menos um teste de integração com arquivo real. |
| TST02 | O corpus inclui, para cada formato de entrada: arquivo válido mínimo, arquivo típico, arquivo grande, arquivo truncado, arquivo com cabeçalho inválido, extensão errada e casos-limite (16 bits, CMYK, transparência, orientação EXIF 1–8, animado, multipágina, perfil ICC). |
| TST03 | O corpus inclui ZIPs maliciosos: zip slip (`../`, absoluto, unidade, UNC, symlink), zip bomb (taxa alta, aninhada, muitos itens), nomes CP437, nomes reservados do Windows, ZIP64, criptografado (ZipCrypto e AES), diretório central corrompido. |
| TST04 | O corpus inclui PDFs: com senha de abertura, com permissões restritas, com formulários, com marcadores, com JavaScript, versões 1.3 a 2.0, linearizado, com xref corrompido, com páginas giradas, com tamanhos de página mistos. |
| TST05 | Verificação de saída: imagens convertidas são decodificadas de volta e comparadas (dimensões exatas; sem perdas: pixels idênticos; com perdas: PSNR ≥ 35 dB a q85 **[P]**). |
| TST06 | Verificação de RN02: todo teste de integração confere o hash do arquivo de entrada antes e depois. |
| TST07 | Verificação de RN07: saída sem "Manter metadados" não contém blocos EXIF, XMP, IPTC nem coordenadas GPS. |
| TST08 | Verificação de RN09 e RN10: nenhum teste de extração maliciosa escreve fora do diretório temporário de destino; limites interrompem e limpam. |
| TST09 | Arquivos do corpus têm origem e licença registradas (`corpus/SOURCES.md`); somente arquivos gerados pelo projeto ou com licença livre. |
| TST10 | Zero rede: (a) desktop: teste de integração roda o app com a rede bloqueada no nível do sistema e um monitor que falha em qualquer tentativa de conexão **[P]**; (b) web: Playwright **[P]** carrega o app, ativa modo offline, executa uma conversão de cada categoria e falha se qualquer requisição sair da origem; (c) CI verifica ausência de dependências de cliente HTTP. |
| TST11 | Testes golden cobrem T01–T15 em claro/escuro × pt-BR/inglês na largura padrão, e T01, T05 e T08 nas larguras da seção 9.8. |
| TST12 | Cobertura do núcleo ≥ 80% (RNF09), medida com `cargo llvm-cov` **[P]**. |

### 14.3 Definição de pronto

Um requisito está pronto quando:
1. Todos os critérios de aceitação passam.
2. Há testes automatizados cobrindo os critérios (quando automatizável).
3. Funciona em Windows, macOS, Linux e web (salvo exceção documentada).
4. Textos estão em pt-BR e inglês.
5. Passa na navegação por teclado e tem semântica para leitor de tela.
6. Não introduz dependência fora da política de licenças.
7. CI verde em todas as plataformas.
8. Documentação atualizada (README, CHANGELOG, este documento se mudou comportamento).

---

## 15. Integração contínua e entrega

### 15.1 Pipeline (GitHub Actions)

| Etapa | Gatilho | Plataformas |
| --- | --- | --- |
| Formatação e análise (`cargo fmt --check`, `clippy`, `dart format`, `flutter analyze`) | Todo push e PR | Linux |
| Licenças e vulnerabilidades (`cargo deny check`, `cargo audit`, verificação Dart) | Todo push e PR | Linux |
| Testes do núcleo + cobertura | Todo push e PR | Linux, Windows, macOS |
| Build WASM do núcleo | Todo push e PR | Linux |
| Testes Flutter (unitários, widget, golden) | Todo push e PR | Linux |
| Build do app (debug) | Todo PR | Windows, macOS, Linux, web |
| Testes de integração + zero rede | Todo PR | Windows, macOS, Linux, web |
| Fuzzing curto (5 min por alvo) | Todo PR **[P]** | Linux |
| Fuzzing longo (1 h por alvo) | Diário **[P]** | Linux |
| Build de release, assinatura, instaladores | Tag `v*` | Windows, macOS, Linux, web |
| Publicação da PWA | Tag `v*` | GitHub Pages |

### 15.2 Versionamento

- Versionamento semântico (`MAJOR.MINOR.PATCH`). MVP sai como `0.1.0`; `1.0.0` quando o MVP estiver estável em todas as plataformas.
- `CHANGELOG.md` no formato *Keep a Changelog*.
- Commits no padrão *Conventional Commits* **[P]** (`feat:`, `fix:`, `docs:`…).
- Builds reproduzíveis: versões de toolchain fixadas (`rust-toolchain.toml`, versão do Flutter registrada em `.fvmrc` ou documentada) e `Cargo.lock`/`pubspec.lock` versionados.

---

## 16. Distribuição

### 16.1 Pacotes

| Plataforma | Formato | Assinatura |
| --- | --- | --- |
| Windows | Instalador `.exe` (Inno Setup **[P]**) e `.msix`; versão portátil `.zip` **[P]** | Assinatura de código (certificado a obter; ver R4) |
| macOS | `.dmg` com binário universal | Developer ID + notarização Apple |
| Linux | AppImage, Flatpak (Flathub), `.deb` | Checksums SHA-256 assinados |
| Web | PWA estática em GitHub Pages | HTTPS da hospedagem |

Todos os artefatos de release são publicados no GitHub Releases com checksums SHA-256.

### 16.2 Atualizações

- Sem verificação automática de versão pela rede (RNF10).
- Atualização manual (baixar nova versão) ou via lojas e gerenciadores (Flathub, Microsoft Store, Homebrew Cask, winget) **[P]**, que fazem a verificação por conta própria, fora do app.
- PWA: o Service Worker atualiza o cache quando o usuário acessa o site com conexão. A interface **DEVE** avisar "Nova versão disponível — recarregar" quando um novo Service Worker estiver pronto **[P]**; isso não é uma requisição do app, é o mecanismo padrão do navegador ao carregar o site. (Ver pendência D1.)

### 16.3 Execução da build web localmente

A build web do Flutter não funciona aberta via `file://`. O README **DEVE** documentar como servi-la localmente com um servidor estático de uma linha (ex.: `python -m http.server`) para quem clonar o código.

---

## 17. Projeto de código aberto

- Repositório público no GitHub.
- Arquivos obrigatórios: `README.md` (o que é, capturas, como instalar, como compilar em cada sistema), `CONTRIBUTING.md` (fluxo de contribuição, padrões de código, como adicionar uma ferramenta), `SECURITY.md`, `CODE_OF_CONDUCT.md` (Contributor Covenant **[P]**), `LICENSE-MIT`, `LICENSE-APACHE`, `CHANGELOG.md`.
- Guia "Como adicionar uma ferramenta", com exemplo passo a passo usando uma ferramenta simples.
- Modelos de issue (bug, sugestão) e de pull request, com checklist da definição de pronto.
- Contribuições aceitas sob a mesma licença dupla (cláusula de contribuição no estilo Apache-2.0, sem CLA **[P]**).
- Build reproduzível documentada, com versões exatas das ferramentas.

---

## 18. Plano de desenvolvimento

| Marco | Entrega | Critério de saída |
| --- | --- | --- |
| **M0 — Ambiente** | Flutter, Rust, gerador da ponte, alvo WASM, ferramentas de compilação do sistema | Versões confirmadas; `flutter doctor` sem erros para a plataforma de desenvolvimento |
| **M1 — Esqueleto** | Repositório com a estrutura da 12.3; fontes e ícone; tela mínima PNG → JPG ponta a ponta | Conversão funciona no desktop de desenvolvimento e na web; núcleo com primeiro teste |
| **M2 — Infraestrutura** | CI completa (15.1), `deny.toml`, teste de zero rede, sistema visual em código (tokens e componentes base) | CI verde nas 4 plataformas; teste de zero rede ativo |
| **M3 — Imagens** | `IMG-01` completo: matriz inteira, opções, edição, lote, pré-visualização, triagem | TST01, TST02, TST05–TST07 passando para imagens |
| **M4 — PDF** | `PDF-01` a `PDF-05` | TST04 passando; grade de miniaturas fluida com 100 páginas |
| **M5 — ZIP** | `ZIP-01` a `ZIP-03` com RN09, RN10, SEG03–SEG05 | TST03 e TST08 passando; fuzzing sem falhas por 1 h |
| **M6 — Acabamento** | Presets, histórico, configurações, i18n, acessibilidade, responsivo, instaladores, PWA | Definição de pronto cumprida para todos os RF de MVP; release `0.1.0` |
| **M7 — Pós-MVP** | 7z, TAR, GZ, XZ, RAR (extração), `ZIP-04`, integração com o sistema operacional | |
| **M8 — Futuro** | `CLR-01` Seletor de cores, `PDF-06` Edição de PDF | Requisitos próprios a especificar |

---

## 19. Riscos

| ID | Risco | Probabilidade | Impacto | Mitigação |
| --- | --- | --- | --- | --- |
| R1 | PDFium em WASM aumenta muito o tamanho da build web ou tem limitações | Média | Médio | Carregar o módulo PDF sob demanda (lazy load); medir no M1; alternativa `pdf.js` só para renderização na web, se a licença (Apache-2.0) e o tamanho compensarem |
| R2 | `flutter_rust_bridge` + WASM + Web Worker com atrito de configuração | Média | Alto | Validar no M1 antes de qualquer ferramenta (esqueleto ponta a ponta) |
| R3 | Desempenho da web abaixo das metas | Média | Médio | Metas web separadas (RNF02); SIMD do WASM quando disponível; workers múltiplos no Pós-MVP |
| R4 | Custo de certificados de assinatura (Windows, Apple Developer) | Alta | Médio | Apple Developer é anual (~US$ 99); Windows: avaliar SignPath (gratuito para open source) **[P]**; até lá, documentar o aviso do SmartScreen |
| R5 | Mantenedor sem experiência em Rust | Alta | Médio | Núcleo com API pequena e bem delimitada; Rust "de usar bibliotecas"; explicações no código; testes extensivos |
| R6 | Dependência com licença incompatível entra indiretamente | Média | Alto | `cargo deny` bloqueando na CI desde o M2 |
| R7 | Gestão de cor (ICC, CMYK) gera cores diferentes do esperado | Média | Baixo | Conversão para sRGB documentada; casos no corpus; avaliar `lcms2` (MIT) ou `qcms` (MIT) **[P]** |
| R8 | Arquivos de fornecedores com variações exóticas (TIFF raros, PDFs quebrados) | Alta | Médio | Corpus crescente a partir de bugs reportados; mensagens de erro claras |
| R9 | Testar macOS sem acesso a um Mac | Média | Médio | CI em runner macOS; testes manuais com voluntários antes de releases |
| R10 | Memória da web insuficiente para arquivos grandes | Média | Médio | Limites específicos da web (RN10, SEG06); mensagem clara sugerindo o app desktop |

---

## 20. Registro de decisões e pendências

### 20.1 Decisões tomadas

| # | Decisão | Alternativas consideradas |
| --- | --- | --- |
| DC01 | 100% offline, sem conta, sem servidor | Processamento em servidor; híbrido |
| DC02 | Flutter na interface + núcleo em Rust | Dart puro |
| DC03 | Licença MIT OR Apache-2.0; dependências só permissivas | GPL/AGPL (permitiria MuPDF, Ghostscript) |
| DC04 | HEIC fora do escopo | libheif (LGPL) |
| DC05 | Sem compressão de arquivos em nenhuma forma | Compressão de PDF e imagens |
| DC06 | ZIP no MVP; 7z, TAR, GZ, XZ e extração de RAR no Pós-MVP | Tudo no MVP |
| DC07 | Limites de extração da RN10 | Limite único de tamanho |
| DC08 | Direção visual "bancada", com vocabulário comum | Vocabulário de ordem de serviço |
| DC09 | Tipografia: IBM Plex Sans Condensed + Ioskeley Mono NL | Univers (comercial) |
| DC10 | Nome: Morsa | Bancada, Crisol, Casulo e outros |
| DC11 | Web distribuída como PWA estática; desktop é o produto principal | Abrir `index.html` via `file://` (não funciona) |
| DC12 | Sem atualização automática pela rede | Checagem de versão opcional |
| DC13 | Desvio do SEG13: a CSP da web permite `'unsafe-eval'` em `script-src`, porque o `flutter_rust_bridge` avalia dois textos fixos da própria biblioteca (expor o módulo WASM e localizar o script dos workers). Nada vindo do usuário ou da rede é avaliado; `connect-src` e `default-src` continuam restritos à própria origem, então o RNF10 não muda. Revisitar a cada atualização do FRB (2026-10-12, Marco 1) | Interceptar as duas chamadas com um script próprio (frágil); ponte web própria sem FRB (mais trabalho) |
| DC14 | macOS sem App Sandbox: o RF10 exige gravar em `Documentos/Morsa` e na pasta da origem, o que o sandbox não permite; a distribuição é por DMG com Developer ID e notarização (§16.1), que não exige sandbox. Nenhuma permissão de rede é pedida (2026-10-12, Marco 2) | Sandbox com pasta escolhida pelo usuário e marcadores de segurança (contraria o padrão do RF10) |
| DC15 | Como o TST10 é aplicado: (a) desktop: no Linux, a integração roda num namespace de rede sem interfaces externas, sob `strace`, e falha em qualquer tentativa IPv4/IPv6 do processo do app; Windows e macOS rodam a mesma integração sem o monitor; (b) web: Playwright offline com controle negativo; (c) `cargo deny` barra clientes de rede no Rust e `scripts/check_dart_deps.dart` barra *imports* de pacotes de rede no Dart. O pacote `http` aparece no grafo Dart por declaração do `file_selector_platform_interface`, mas não é importado por nenhum pacote e não entra no app (2026-10-12, Marco 2) | Monitor nos três sistemas (sem ferramenta equivalente simples no Windows e macOS da CI); barrar `http` pela simples presença (exigiria trocar o seletor de arquivos oficial) |
| DC16 | Escala de texto maior que a do §9.3, a pedido da usuária (leitura difícil nos tamanhos dos protótipos): corpo 16 px (era 13), ficha e tabela 15 (era 12), metadado 14 (era 11), título de painel 19 (era 16), título de tela 34 (era 30). Controles e linhas de tabela passam de 28 para 34 px; ficha e itens de menu, de 26 para 32; barra de status, de 24 para 30; painel de parâmetros, de 280 para 340 px. Tudo definido em `app/lib/design/theme.dart` (2026-10-13) | Zoom de interface nas Configurações (a avaliar no Marco 6) |

### 20.2 Pendências

| # | Pendência | Responsável | Prazo |
| --- | --- | --- | --- |
| D1 | Confirmar a resposta ao comentário sobre atualização: manter só manual/lojas, ou permitir aviso de nova versão da PWA (16.2) | Produto | Antes do M6 |
| D2 | Verificar disponibilidade do nome "Morsa" em GitHub, pub.dev, crates.io, lojas e INPI | Produto | Antes do M1 |
| D3 | Definir o domínio ou subcaminho da PWA no GitHub Pages | Produto | Antes do M6 |
| D4 | Telas restantes no Claude Design: `IMG-01`, `PDF-03/05`, `PDF-04/01`, `ZIP-02`, Histórico, `SYS-00`, layouts estreitos | Design | Antes do marco de cada ferramenta |
| D5 | Obter certificados de assinatura (R4) | Produto | Antes do M6 |
| D6 | Revisar todos os valores marcados com **[P]** durante a implementação | Desenvolvimento | Contínuo |
| D7 | Modelo de múltiplas ordens abertas: abas, lista de rascunhos ou uma por vez | Design | Antes do M3 |

---

## Apêndice A — Catálogo de mensagens de erro

Cada erro tem código estável, mensagem ao usuário (pt-BR) e sugestão. As mensagens em inglês ficam no arquivo ARB correspondente. O código aparece discretamente no relatório copiado, para suporte, não em destaque na interface.

| Código | Situação | Mensagem | Sugestão |
| --- | --- | --- | --- |
| E-101 | Conversão fora da matriz | Esta conversão não é suportada. | Escolha outro formato de destino. |
| E-102 | Formato não reconhecido | O formato deste arquivo não é suportado. | Confira se o arquivo é do tipo esperado. |
| E-103 | Arquivo vazio | O arquivo está vazio (0 bytes). | Peça o arquivo novamente a quem o enviou. |
| E-104 | Opção inválida | Uma opção está fora do intervalo permitido. | Ajuste o valor destacado. |
| E-105 | Arquivo truncado/corrompido | O arquivo terminou antes do esperado; pode estar corrompido. | Tente reexportar o arquivo na origem. |
| E-106 | Imagem grande demais | A imagem é grande demais para ser processada com segurança ({largura} × {altura}). | Reduza a imagem em outro programa ou use o app desktop. |
| E-107 | Tempo esgotado | A operação demorou mais que o limite e foi interrompida. | Tente com menos páginas ou um arquivo menor. |
| E-108 | Animação descartada (aviso) | Somente o primeiro quadro foi usado. | — |
| E-109 | Página extra descartada (aviso) | Somente a primeira página do TIFF foi usada. | — |
| E-110 | Recursos externos no SVG (aviso) | Imagens ou fontes externas do SVG foram ignoradas. | — |
| E-199 | Erro interno | Ocorreu um erro inesperado ao processar este arquivo. | Copie o relatório e informe no repositório do projeto. |
| E-201 | Sem permissão de escrita | Não é possível salvar na pasta escolhida. | Escolha outra pasta de destino. |
| E-202 | Senha incorreta | Senha incorreta. | Tente novamente. |
| E-203 | Disco cheio | Não há espaço suficiente em disco. | Libere espaço e tente novamente. |
| E-204 | Arquivo de origem indisponível | O arquivo original não pode mais ser lido (foi movido ou apagado). | Adicione o arquivo novamente. |
| E-205 | PDF danificado | O PDF está danificado e não pôde ser aberto. | Tente abrir e salvar novamente em outro leitor de PDF. |
| E-206 | Página grande demais para renderizar | A página ficaria grande demais nessa resolução. | Reduza o DPI. |
| E-207 | Memória insuficiente | Não há memória suficiente para este arquivo. | Feche outros programas ou use o app desktop. |
| E-301 | Compactado danificado | O arquivo compactado está danificado. | Peça o arquivo novamente a quem o enviou. |
| E-302 | Limite total de extração | A extração passou do limite de {limite} e foi interrompida. Nada foi mantido. | Aumente o limite em Configurações, se confiar no arquivo. |
| E-303 | Espaço mínimo em disco | A extração foi interrompida para manter 1 GB livre no disco. | Libere espaço e tente novamente. |
| E-304 | Itens demais | O arquivo tem mais de 100 mil itens. | Extraia apenas os itens selecionados. |
| E-305 | Caminho perigoso (por item) | Este item tentava gravar fora da pasta de destino e foi ignorado. | — |
| E-306 | Taxa de compressão suspeita (pergunta) | Este item expande mais de 1000 vezes. Isso é incomum e pode indicar um arquivo malicioso. | Cancelar (padrão) ou Continuar. |
| E-307 | Método de compressão não suportado | Este item usa um método de compressão não suportado. | — |
| E-308 | Documento, não pacote | Este arquivo é um documento (ex.: DOCX), não um pacote para extrair. | — |

## Apêndice B — Atalhos de teclado

`Mod` = `Ctrl` no Windows e Linux, `Cmd` no macOS.

| Atalho | Ação | Contexto |
| --- | --- | --- |
| `Mod+O` | Abrir arquivos | Global |
| `Mod+V` | Colar arquivos ou imagem | Global |
| `Mod+K` | Buscar ferramenta | Global |
| `Mod+H` | Início | Global |
| `Mod+L` | Histórico | Global |
| `Mod+,` | Configurações | Global |
| `Mod+Enter` | Executar a ordem (Converter, Juntar…) | Ferramenta |
| `Esc` | Cancelar ordem em execução / fechar diálogo / cancelar arraste | Global |
| `↑` `↓` | Mover entre itens ou grupos | Listas |
| `Espaço` | Incluir/excluir item ou grupo | Listas, triagem |
| `Del` / `Backspace` | Remover item ou página | Listas, grade |
| `Mod+A` | Selecionar tudo | Listas, grade |
| `Alt+↑` `Alt+↓` (`Alt+setas` em grade) | Mover item/página selecionada | Listas ordenáveis, grade |
| `R` / `Shift+R` | Girar página 90° / −90° | `PDF-05` |
| `Mod+D` | Duplicar página | `PDF-05` |
| `Mod+Z` / `Mod+Shift+Z` | Desfazer / refazer | `PDF-05`, edição de imagem |
| `F1` | Mostrar atalhos | Global **[P]** |

Atalhos de uma letra (`R`) só funcionam quando o foco está na grade, nunca em campos de texto. Na web, atalhos que conflitam com o navegador (`Mod+L`, `Mod+H`) usam alternativas `Alt+Shift+L` e `Alt+Shift+H` **[P]**.

## Apêndice C — Regras de nomes de saída

1. **Base:** nome do arquivo de origem sem extensão.
2. **Extensão:** sempre a do formato de destino, em minúsculas (RN15).
3. **Sufixos de operação:** `-p<NNN>` (página de PDF), `-<NN>` (parte de divisão), `-reordenado`, `-extraido`.
4. **Conflito:** se o nome existir, acrescentar ` (1)`, ` (2)`… antes da extensão (RN03). Conflitos dentro do próprio lote (ex.: `foto.png` e `foto.jpg` → `foto.webp`) seguem a mesma regra, pela ordem dos itens.
5. **Saneamento (todas as plataformas, para portabilidade):**
   - Substituir `< > : " / \ | ? *` e caracteres de controle (0x00–0x1F) por `_`.
   - Remover espaços e pontos no final do nome.
   - Nomes reservados do Windows (`CON`, `PRN`, `AUX`, `NUL`, `COM1`–`COM9`, `LPT1`–`LPT9`, com ou sem extensão) recebem `_` no final.
   - Limitar a 200 bytes UTF-8 **[P]**, cortando na fronteira de caractere e preservando a extensão.
   - Normalizar para Unicode NFC.
6. **Nome vazio após saneamento:** usar `arquivo`.
7. **Saneamentos aplicados** são listados no resumo da ordem.

## Apêndice D — Assinaturas de formato (magic bytes)

| Formato | Deslocamento | Bytes (hex) | Observação |
| --- | --- | --- | --- |
| PNG | 0 | `89 50 4E 47 0D 0A 1A 0A` | |
| JPG | 0 | `FF D8 FF` | |
| GIF | 0 | `47 49 46 38 37 61` ou `47 49 46 38 39 61` | `GIF87a` / `GIF89a` |
| WebP | 0 e 8 | `52 49 46 46` … `57 45 42 50` | `RIFF....WEBP` |
| BMP | 0 | `42 4D` | `BM` |
| TIFF | 0 | `49 49 2A 00` ou `4D 4D 00 2A` | little / big endian; BigTIFF `2B` |
| ICO | 0 | `00 00 01 00` | CUR (`00 00 02 00`) recusado |
| SVG | — | texto contendo `<svg` nos primeiros 64 KB, após BOM, declaração XML e comentários | Validado como XML |
| PDF | 0 a 1023 | `25 50 44 46 2D` | `%PDF-`, aceito em qualquer posição no 1º KB |
| ZIP | 0 | `50 4B 03 04`, `50 4B 05 06` (vazio) ou `50 4B 07 08` | Também procurar o diretório central no fim (ZIPs com prefixo, ex.: autoextraíveis) |
| 7z | 0 | `37 7A BC AF 27 1C` | Pós-MVP |
| GZ | 0 | `1F 8B` | Pós-MVP |
| XZ | 0 | `FD 37 7A 58 5A 00` | Pós-MVP |
| TAR | 257 | `75 73 74 61 72` | `ustar`; TAR antigos sem assinatura usam extensão + validação do checksum do cabeçalho |
| RAR | 0 | `52 61 72 21 1A 07 00` (v4) ou `52 61 72 21 1A 07 01 00` (v5) | Pós-MVP |

---

*Fim do documento.*
