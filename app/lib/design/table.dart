import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'controls.dart';
import 'focus_ring.dart';
import 'theme.dart';

/// Largura de uma coluna de tabela: fixa em px ou flexível.
class MorsaColumn {
  const MorsaColumn.fixed(this.width, {this.alignEnd = false}) : flex = null;
  const MorsaColumn.flex([this.flex = 1, this.alignEnd = false]) : width = null;

  final double? width;
  final int? flex;

  /// Números alinham à direita (§9.3).
  final bool alignEnd;
}

/// Monta as células de uma linha conforme as colunas.
Widget _cells(List<MorsaColumn> columns, List<Widget> cells) {
  assert(columns.length == cells.length, 'uma célula por coluna');
  return Row(
    children: [
      for (var i = 0; i < columns.length; i++)
        _cell(
          columns[i],
          Align(
            alignment: columns[i].alignEnd
                ? Alignment.centerRight
                : Alignment.centerLeft,
            child: cells[i],
          ),
        ),
    ],
  );
}

Widget _cell(MorsaColumn c, Widget child) => c.width != null
    ? SizedBox(width: c.width, child: child)
    : Expanded(flex: c.flex!, child: child);

/// Cabeçalho de tabela: 28 px, 11 px / 700 em caixa alta, régua de tinta.
class MorsaTableHeader extends StatelessWidget {
  const MorsaTableHeader({
    super.key,
    required this.columns,
    required this.labels,
  });

  final List<MorsaColumn> columns;
  final List<String> labels;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    return Container(
      height: 28,
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: t.ink)),
      ),
      child: DefaultTextStyle(
        style: TextStyle(
          fontSize: 11,
          fontWeight: FontWeight.w700,
          letterSpacing: 0.66,
          color: t.ink,
        ),
        child: _cells(columns, [
          for (final l in labels)
            Semantics(header: true, child: Text(l.toUpperCase())),
        ]),
      ),
    );
  }
}

/// Linha de tabela de 28 px (DOC-01 §5).
///
/// - Hover: tom de papel (`sunk`).
/// - Selecionada: tom mais forte (`sel`) e caixa marcada.
/// - Foco: anel azul por dentro (−2 px), para a rolagem não cortar.
///
/// Espaço marca ou desmarca a linha; Enter aciona `onActivate`.
class MorsaTableRow extends StatefulWidget {
  const MorsaTableRow({
    super.key,
    required this.columns,
    required this.cells,
    this.selected = false,
    this.onSelectedChanged,
    this.onActivate,
    this.semanticLabel,
  });

  /// Colunas de dados (a coluna da caixa de seleção é acrescentada antes).
  final List<MorsaColumn> columns;
  final List<Widget> cells;
  final bool selected;
  final ValueChanged<bool>? onSelectedChanged;
  final VoidCallback? onActivate;
  final String? semanticLabel;

  @override
  State<MorsaTableRow> createState() => _MorsaTableRowState();
}

class _MorsaTableRowState extends State<MorsaTableRow> {
  bool _hover = false;
  bool _focus = false;

  void _toggle() => widget.onSelectedChanged?.call(!widget.selected);

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final bg = widget.selected ? t.sel : (_hover ? t.sunk : null);
    return Semantics(
      selected: widget.selected,
      label: widget.semanticLabel,
      onTap: widget.onSelectedChanged == null ? null : _toggle,
      child: FocusableActionDetector(
        onShowHoverHighlight: (v) => setState(() => _hover = v),
        onShowFocusHighlight: (v) => setState(() => _focus = v),
        shortcuts: const {
          SingleActivator(LogicalKeyboardKey.space): _ToggleIntent(),
          SingleActivator(LogicalKeyboardKey.enter): ActivateIntent(),
        },
        actions: {
          _ToggleIntent: CallbackAction<_ToggleIntent>(
            onInvoke: (_) => _toggle(),
          ),
          ActivateIntent: CallbackAction<ActivateIntent>(
            onInvoke: (_) => widget.onActivate?.call(),
          ),
        },
        child: GestureDetector(
          behavior: HitTestBehavior.opaque,
          excludeFromSemantics: true,
          onTap: widget.onSelectedChanged == null ? null : _toggle,
          onDoubleTap: widget.onActivate,
          child: FocusRing(
            visible: _focus,
            inset: true,
            child: Container(
              height: 28,
              decoration: BoxDecoration(
                color: bg,
                border: Border(bottom: BorderSide(color: t.weak)),
              ),
              child: DefaultTextStyle(
                style: Theme.of(context).textTheme.bodySmall!,
                maxLines: 1,
                overflow: TextOverflow.ellipsis,
                child: Row(
                  children: [
                    SizedBox(
                      width: 28,
                      child: Padding(
                        padding: const EdgeInsets.only(left: 4),
                        child: Align(
                          alignment: Alignment.centerLeft,
                          child: CheckMark(value: widget.selected),
                        ),
                      ),
                    ),
                    Expanded(child: _cells(widget.columns, widget.cells)),
                  ],
                ),
              ),
            ),
          ),
        ),
      ),
    );
  }
}

class _ToggleIntent extends Intent {
  const _ToggleIntent();
}

/// Nota de rodapé da tabela (11 px, `ink2`), como "* 003 · ICO → PNG: …".
class MorsaTableNote extends StatelessWidget {
  const MorsaTableNote(this.text, {super.key});

  final String text;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    return Container(
      constraints: const BoxConstraints(minHeight: 28),
      padding: const EdgeInsets.fromLTRB(72, 4, 0, 4),
      alignment: Alignment.centerLeft,
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: t.weak)),
      ),
      child: Text(text, style: TextStyle(fontSize: 11, color: t.ink2)),
    );
  }
}

/// Linha de totais: régua forte de 2 px em cima, texto em 700.
class MorsaTableTotal extends StatelessWidget {
  const MorsaTableTotal({
    super.key,
    required this.columns,
    required this.cells,
  });

  final List<MorsaColumn> columns;
  final List<Widget> cells;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    return Container(
      height: 28,
      decoration: BoxDecoration(
        border: Border(top: BorderSide(color: t.ink, width: 2)),
      ),
      child: DefaultTextStyle(
        style: Theme.of(context).textTheme.bodySmall!
            .copyWith(fontWeight: FontWeight.w700),
        child: Row(
          children: [
            const SizedBox(width: 28),
            Expanded(child: _cells(columns, cells)),
          ],
        ),
      ),
    );
  }
}
