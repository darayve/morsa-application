// Galeria dos componentes base (DOC-01 §5), só para desenvolvimento.
// Não faz parte do app: serve para conferir cada componente e seus estados
// nos dois temas. Os textos são de exemplo, copiados do protótipo.

import 'package:flutter/material.dart';
import 'package:morsa/app/platform.dart';
import 'package:morsa/design/controls.dart';
import 'package:morsa/design/feedback.dart';
import 'package:morsa/design/status_bar.dart';
import 'package:morsa/design/surfaces.dart';
import 'package:morsa/design/table.dart';
import 'package:morsa/design/theme.dart';
import 'package:morsa/design/widgets.dart';

class ComponentGallery extends StatefulWidget {
  const ComponentGallery({super.key, this.onToggleTheme});

  final VoidCallback? onToggleTheme;

  @override
  State<ComponentGallery> createState() => _ComponentGalleryState();
}

class _ComponentGalleryState extends State<ComponentGallery> {
  final _normal = TextEditingController(text: 'Qualidade 85');
  final _dpi = TextEditingController(text: '300');
  final _dpiError = TextEditingController(text: '1200');
  final _disabled = TextEditingController(text: '~/Saída');
  final _menu = MenuController();

  bool? _keep = true;
  bool _unchecked = false;
  String _format = 'PNG';
  String _select = 'png';
  final _selected = <int>{3};
  int _thumb = 1;

  @override
  void dispose() {
    for (final c in [_normal, _dpi, _dpiError, _disabled]) {
      c.dispose();
    }
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final text = Theme.of(context).textTheme;
    final mod = modKeyLabel;

    const cols = [
      MorsaColumn.fixed(52),
      MorsaColumn.flex(),
      MorsaColumn.fixed(120),
      MorsaColumn.fixed(120),
      MorsaColumn.fixed(150, alignEnd: true),
      MorsaColumn.fixed(150, alignEnd: true),
    ];
    final rows = [
      (
        '001',
        'foto-fornecedor-lote-0012-revisao-final-aprovada.tiff',
        'TIFF',
        '→ PNG',
        '12 940,1 KB',
        ItemStatus.queued,
        'Na fila',
      ),
      (
        '002',
        'capa.png',
        'PNG',
        '→ WebP',
        '1 204,3 KB',
        ItemStatus.running,
        'Executando',
      ),
      (
        '003',
        'logo.ico',
        'ICO',
        '→ PNG*',
        '88,0 KB',
        ItemStatus.done,
        'Concluído',
      ),
      (
        '004',
        'planta-B.svg',
        'SVG',
        '→ PNG',
        '42,7 KB',
        ItemStatus.queued,
        'Na fila',
      ),
      (
        '005',
        'scan-03.bmp',
        'BMP',
        '→ JPG',
        '6 120,0 KB',
        ItemStatus.failed,
        'Falha',
      ),
    ];

    Widget section(String title, List<Widget> children) => Padding(
      padding: const EdgeInsets.only(bottom: 32),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SectionLabel(title),
          const SizedBox(height: 12),
          ...children,
        ],
      ),
    );

    Widget labeled(String label, Widget child) => Padding(
      padding: const EdgeInsets.only(bottom: 12),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 120,
            height: kControlHeight,
            child: Align(
              alignment: Alignment.centerLeft,
              child: Text(
                label,
                style: text.bodySmall!.copyWith(color: t.ink2),
              ),
            ),
          ),
          Flexible(child: child),
        ],
      ),
    );

    return Scaffold(
      body: Column(
        children: [
          Expanded(
            child: SingleChildScrollView(
              padding: const EdgeInsets.fromLTRB(32, 24, 32, 24),
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  Row(
                    children: [
                      Expanded(
                        child: Text(
                          'DOC-01 · Componentes base',
                          style: text.headlineMedium,
                        ),
                      ),
                      MorsaButton(
                        label: 'Alternar tema',
                        shortcut: '$mod T',
                        onPressed: widget.onToggleTheme,
                      ),
                    ],
                  ),
                  const SizedBox(height: 24),
                  section('Botões', [
                    labeled(
                      'Primário',
                      Wrap(
                        spacing: 16,
                        runSpacing: 12,
                        children: [
                          MorsaButton(
                            label: 'Converter',
                            primary: true,
                            onPressed: () {},
                          ),
                          const MorsaButton(
                            label: 'Converter',
                            primary: true,
                            onPressed: null,
                          ),
                        ],
                      ),
                    ),
                    labeled(
                      'Secundário',
                      Wrap(
                        spacing: 16,
                        runSpacing: 12,
                        children: [
                          MorsaButton(label: 'Abrir pasta', onPressed: () {}),
                          const MorsaButton(
                            label: 'Abrir pasta',
                            onPressed: null,
                          ),
                        ],
                      ),
                    ),
                    labeled(
                      'Link',
                      Wrap(
                        spacing: 16,
                        runSpacing: 12,
                        children: [
                          MorsaLink(
                            label: 'Copiar relatório',
                            onPressed: () {},
                          ),
                          const MorsaLink(
                            label: 'Copiar relatório',
                            onPressed: null,
                          ),
                        ],
                      ),
                    ),
                    labeled(
                      'Com atalho',
                      MorsaButton(
                        label: 'Abrir arquivos…',
                        shortcut: '$mod O',
                        onPressed: () {},
                      ),
                    ),
                  ]),
                  section('Campos', [
                    labeled(
                      'Normal',
                      MorsaTextField(
                        controller: _normal,
                        label: 'Normal',
                        width: 240,
                      ),
                    ),
                    labeled(
                      'Numérico',
                      MorsaTextField(
                        controller: _dpi,
                        label: 'DPI',
                        suffix: 'DPI',
                        numeric: true,
                        width: 240,
                      ),
                    ),
                    labeled(
                      'Erro',
                      MorsaTextField(
                        controller: _dpiError,
                        label: 'DPI',
                        suffix: 'DPI',
                        numeric: true,
                        width: 240,
                        errorText: 'Fora do intervalo: use de 72 a 600.',
                      ),
                    ),
                    labeled(
                      'Desabilitado',
                      MorsaTextField(
                        controller: _disabled,
                        label: 'Pasta',
                        enabled: false,
                        width: 240,
                      ),
                    ),
                    labeled(
                      'Seletor',
                      MorsaSelect<String>(
                        label: 'Formato de destino',
                        width: 240,
                        value: _select,
                        onChanged: (v) => setState(() => _select = v),
                        options: const [
                          MorsaOption('png', 'PNG · sem perdas'),
                          MorsaOption('jpg', 'JPG · com perdas'),
                          MorsaOption('webp', 'WebP'),
                          MorsaOption('tiff', 'TIFF', enabled: false),
                        ],
                      ),
                    ),
                    labeled(
                      'Segmentado',
                      MorsaSegmented<String>(
                        label: 'Formato',
                        value: _format,
                        onChanged: (v) => setState(() => _format = v),
                        options: const [
                          MorsaOption('PNG', 'PNG'),
                          MorsaOption('JPG', 'JPG'),
                          MorsaOption('WebP', 'WebP'),
                          MorsaOption('TIFF', 'TIFF', enabled: false),
                        ],
                      ),
                    ),
                  ]),
                  section('Seleção e menu', [
                    Wrap(
                      spacing: 48,
                      runSpacing: 24,
                      children: [
                        Column(
                          crossAxisAlignment: CrossAxisAlignment.start,
                          children: [
                            MorsaCheckbox(
                              value: _keep,
                              label: 'Manter metadados',
                              onChanged: (v) => setState(() => _keep = v),
                            ),
                            const SizedBox(height: 8),
                            MorsaCheckbox(
                              value: _unchecked,
                              label: 'Desmarcado',
                              onChanged: (v) => setState(() => _unchecked = v),
                            ),
                            const SizedBox(height: 8),
                            MorsaCheckbox(
                              value: null,
                              label: 'Parcial',
                              onChanged: (_) => setState(() => _keep = null),
                            ),
                            const SizedBox(height: 8),
                            const MorsaCheckbox(
                              value: false,
                              label: 'Desabilitado',
                              onChanged: null,
                            ),
                          ],
                        ),
                        MenuAnchor(
                          controller: _menu,
                          style: morsaMenuStyle(context),
                          menuChildren: [
                            MorsaMenuItem(
                              label: 'Girar 90°',
                              shortcut: 'R',
                              onPressed: () {},
                            ),
                            MorsaMenuItem(
                              label: 'Duplicar',
                              shortcut: '$mod D',
                              onPressed: () {},
                            ),
                            const MorsaMenuItem(
                              label: 'Extrair página',
                              shortcut: 'E',
                              onPressed: null,
                            ),
                            const MorsaMenuDivider(),
                            MorsaMenuItem(
                              label: 'Remover',
                              shortcut: 'Del',
                              onPressed: () {},
                            ),
                          ],
                          builder: (context, c, _) => MorsaButton(
                            label: 'Menu de contexto…',
                            onPressed: () => c.isOpen ? c.close() : c.open(),
                          ),
                        ),
                      ],
                    ),
                  ]),
                  section('Linhas de tabela · 28 px', [
                    MorsaTableHeader(
                      columns: [const MorsaColumn.fixed(32), ...cols],
                      labels: const [
                        '',
                        'Item',
                        'Arquivo',
                        'Origem',
                        'Destino',
                        'Tamanho',
                        'Situação',
                      ],
                    ),
                    for (final (i, r) in rows.indexed)
                      MorsaTableRow(
                        columns: cols,
                        selected: _selected.contains(i + 1),
                        onSelectedChanged: (v) => setState(
                          () => v
                              ? _selected.add(i + 1)
                              : _selected.remove(i + 1),
                        ),
                        semanticLabel: '${r.$1} ${r.$2}',
                        cells: [
                          Text(r.$1, style: TextStyle(color: t.ink2)),
                          Text(r.$2, overflow: TextOverflow.ellipsis),
                          Text(r.$3),
                          Text(r.$4),
                          Text(r.$5),
                          StatusText(r.$6, r.$7),
                        ],
                      ),
                    const MorsaTableNote(
                      '* 003 · ICO → PNG: ICO só é aceito como entrada nesta saída; '
                      'o maior quadro (256 px) foi usado. · 005 · O arquivo terminou '
                      'antes do esperado; pode estar corrompido.',
                    ),
                    MorsaTableTotal(
                      columns: cols,
                      cells: const [
                        SizedBox(),
                        Text('5 itens · 1 concluído · 1 falha'),
                        SizedBox(),
                        SizedBox(),
                        Text('20 395,1 KB'),
                        SizedBox(),
                      ],
                    ),
                  ]),
                  Wrap(
                    spacing: 32,
                    runSpacing: 32,
                    children: [
                      SizedBox(
                        width: 340,
                        child: section('Ficha rótulo → valor', const [
                          FichaRow(label: 'Ferramenta', value: Text('IMG-01')),
                          FichaRow(label: 'Versão', value: Text('0.1.0')),
                          FichaRow(label: 'Rede', value: Text('Sem acesso')),
                          FichaRow(label: 'Conta', value: Text('Inexistente')),
                          FichaRow(
                            label: 'Licença',
                            value: Text('Código aberto'),
                            last: true,
                          ),
                        ]),
                      ),
                      SizedBox(
                        width: 340,
                        child: section('Progresso', [
                          for (final (v, s, l) in [
                            (null, ItemStatus.queued, 'Na fila'),
                            (0.46, ItemStatus.running, '46%'),
                            (1.0, ItemStatus.done, 'Concluído'),
                            (0.31, ItemStatus.failed, 'Falha · 31%'),
                          ])
                            Padding(
                              padding: const EdgeInsets.only(bottom: 10),
                              child: Row(
                                children: [
                                  Expanded(child: MorsaProgressBar(value: v)),
                                  const SizedBox(width: 12),
                                  SizedBox(width: 124, child: StatusText(s, l)),
                                ],
                              ),
                            ),
                          const SizedBox(height: 6),
                          const MorsaProgressBar(value: 0.62, large: true),
                          const SizedBox(height: 10),
                          Row(
                            children: [
                              Text(
                                '3 de 5 arquivos',
                                style: TextStyle(
                                  fontSize: kFsMeta,
                                  color: t.ink2,
                                ),
                              ),
                              const Spacer(),
                              Text(
                                '~4 s',
                                style: TextStyle(
                                  fontSize: kFsMeta,
                                  color: t.ink2,
                                ),
                              ),
                            ],
                          ),
                        ]),
                      ),
                      SizedBox(
                        width: 340,
                        child: section('Avisos', const [
                          MorsaNotice(
                            kind: NoticeKind.attention,
                            title: 'Atenção · ZIP-02',
                            body:
                                'contratos.zip está protegido por senha. Informe a '
                                'senha para extrair; ela não é guardada.',
                          ),
                          SizedBox(height: 12),
                          MorsaNotice(
                            kind: NoticeKind.failure,
                            title: 'Falha · item 005',
                            body:
                                'scan-03.bmp terminou antes do esperado. Tente '
                                'reexportar o arquivo na origem.',
                          ),
                        ]),
                      ),
                    ],
                  ),
                  Wrap(
                    spacing: 32,
                    runSpacing: 32,
                    children: [
                      SizedBox(
                        width: 560,
                        child: section('Área de soltar', [
                          MorsaDropArea(
                            title: 'Solte arquivos em qualquer lugar da janela',
                            orLabel: 'ou',
                            action: MorsaButton(
                              label: 'Abrir arquivos…',
                              shortcut: '$mod O',
                              onPressed: () {},
                            ),
                            formats:
                                'PNG JPG WebP GIF BMP TIFF ICO SVG · PDF · ZIP',
                          ),
                        ]),
                      ),
                      SizedBox(
                        width: 400,
                        child: section('Miniaturas', [
                          Row(
                            crossAxisAlignment: CrossAxisAlignment.start,
                            children: [
                              MorsaThumbnail(
                                reference: 'A1',
                                categoryColor: t.pdf,
                                selected: _thumb == 0,
                                onTap: () => setState(() => _thumb = 0),
                              ),
                              const SizedBox(width: 12),
                              MorsaThumbnail(
                                reference: 'A2',
                                detail: '☒',
                                categoryColor: t.pdf,
                                selected: _thumb == 1,
                                onTap: () => setState(() => _thumb = 1),
                              ),
                              const SizedBox(width: 12),
                              const InsertionMarker(),
                              const SizedBox(width: 12),
                              MorsaThumbnail(
                                reference: 'C1',
                                detail: '↻ 90°',
                                categoryColor: t.img,
                                selected: _thumb == 2,
                                onTap: () => setState(() => _thumb = 2),
                              ),
                            ],
                          ),
                        ]),
                      ),
                    ],
                  ),
                ],
              ),
            ),
          ),
          const MorsaStatusBar(),
        ],
      ),
    );
  }
}
