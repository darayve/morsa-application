import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'focus_ring.dart';
import 'theme.dart';
import 'widgets.dart';

/// Caixa de seleção de 12 px (DOC-01 §5): marcada (tinta + ✕), desmarcada,
/// parcial (traço), foco (anel por fora) e desabilitada.
///
/// `value` nulo é o estado parcial. Clicar no parcial marca tudo.
class MorsaCheckbox extends StatefulWidget {
  const MorsaCheckbox({
    super.key,
    required this.value,
    required this.onChanged,
    this.label,
  });

  final bool? value;
  final ValueChanged<bool>? onChanged;
  final String? label;

  @override
  State<MorsaCheckbox> createState() => _MorsaCheckboxState();
}

class _MorsaCheckboxState extends State<MorsaCheckbox> {
  bool _focus = false;

  bool get _enabled => widget.onChanged != null;

  void _toggle() => widget.onChanged?.call(widget.value != true);

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final label = widget.label;
    return Semantics(
      checked: widget.value == true,
      mixed: widget.value == null,
      enabled: _enabled,
      focusable: _enabled,
      focused: _focus,
      label: label,
      onTap: _enabled ? _toggle : null,
      excludeSemantics: true,
      child: FocusableActionDetector(
        enabled: _enabled,
        mouseCursor: _enabled
            ? SystemMouseCursors.click
            : SystemMouseCursors.basic,
        onShowFocusHighlight: (v) => setState(() => _focus = v),
        shortcuts: const {
          SingleActivator(LogicalKeyboardKey.space): ActivateIntent(),
        },
        actions: {
          ActivateIntent: CallbackAction<ActivateIntent>(
            onInvoke: (_) => _toggle(),
          ),
        },
        child: GestureDetector(
          behavior: HitTestBehavior.opaque,
          excludeFromSemantics: true,
          onTap: _enabled ? _toggle : null,
          child: Row(
            mainAxisSize: MainAxisSize.min,
            children: [
              FocusRing(
                visible: _focus,
                child: CheckMark(value: widget.value, enabled: _enabled),
              ),
              if (label != null) ...[
                const SizedBox(width: 8),
                Text(
                  label,
                  style: Theme.of(context).textTheme.bodySmall!
                      .copyWith(color: _enabled ? t.ink : t.dis),
                ),
              ],
            ],
          ),
        ),
      ),
    );
  }
}

/// Só o desenho da caixa de 12 px, sem interação (usado também nas linhas
/// de tabela).
class CheckMark extends StatelessWidget {
  const CheckMark({super.key, required this.value, this.enabled = true});

  final bool? value;
  final bool enabled;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final checked = value == true;
    return Container(
      width: 12,
      height: 12,
      alignment: Alignment.center,
      decoration: BoxDecoration(
        color: checked ? (enabled ? t.ink : t.dis) : null,
        border: Border.all(color: enabled ? t.ctrl : t.dis),
      ),
      child: switch (value) {
        true => Text(
          '✕',
          style: TextStyle(fontSize: 10, height: 1, color: t.paper),
        ),
        null => Container(width: 6, height: 2, color: enabled ? t.ink : t.dis),
        false => null,
      },
    );
  }
}

/// Uma opção de [MorsaSegmented] ou [MorsaSelect].
class MorsaOption<T> {
  const MorsaOption(this.value, this.label, {this.enabled = true});

  final T value;
  final String label;
  final bool enabled;
}

/// Controle segmentado (DOC-01 §5): a opção escolhida é tinta cheia; as
/// outras ficam em contorno, com hover em tom de papel. Setas ← → mudam a
/// escolha, como num grupo de rádio.
class MorsaSegmented<T> extends StatefulWidget {
  const MorsaSegmented({
    super.key,
    required this.options,
    required this.value,
    required this.onChanged,
    this.label,
  });

  final List<MorsaOption<T>> options;
  final T value;
  final ValueChanged<T>? onChanged;

  /// Nome do grupo para leitores de tela.
  final String? label;

  @override
  State<MorsaSegmented<T>> createState() => _MorsaSegmentedState<T>();
}

class _MorsaSegmentedState<T> extends State<MorsaSegmented<T>> {
  bool _focus = false;
  int? _hover;

  void _move(int delta) {
    final enabled = [
      for (final o in widget.options)
        if (o.enabled) o,
    ];
    if (enabled.isEmpty || widget.onChanged == null) return;
    final i = enabled.indexWhere((o) => o.value == widget.value);
    final next = enabled[(i + delta).clamp(0, enabled.length - 1)];
    if (next.value != widget.value) widget.onChanged!(next.value);
  }

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final text = Theme.of(context).textTheme.bodyMedium!;
    final groupEnabled = widget.onChanged != null;

    final segments = <Widget>[];
    for (var i = 0; i < widget.options.length; i++) {
      final o = widget.options[i];
      final selected = o.value == widget.value;
      final enabled = groupEnabled && o.enabled;
      final bg = selected
          ? t.ink
          : (enabled && _hover == i ? t.sunk : Colors.transparent);
      final fg = selected ? t.paper : (enabled ? t.ink : t.dis);
      segments.add(
        Semantics(
          button: true,
          selected: selected,
          enabled: enabled,
          inMutuallyExclusiveGroup: true,
          label: o.label,
          onTap: enabled ? () => widget.onChanged!(o.value) : null,
          excludeSemantics: true,
          child: MouseRegion(
            cursor: enabled && !selected
                ? SystemMouseCursors.click
                : SystemMouseCursors.basic,
            onEnter: (_) => setState(() => _hover = i),
            onExit: (_) => setState(() => _hover = null),
            child: GestureDetector(
              behavior: HitTestBehavior.opaque,
              excludeFromSemantics: true,
              onTap: enabled ? () => widget.onChanged!(o.value) : null,
              child: Container(
                padding: const EdgeInsets.symmetric(horizontal: 12),
                alignment: Alignment.center,
                decoration: BoxDecoration(
                  color: bg,
                  border: i == 0
                      ? null
                      : Border(left: BorderSide(color: t.ctrl)),
                ),
                child: Text(
                  o.label,
                  style: text.copyWith(
                    color: fg,
                    fontWeight: selected ? FontWeight.w500 : FontWeight.w400,
                  ),
                ),
              ),
            ),
          ),
        ),
      );
    }

    return Semantics(
      label: widget.label,
      container: true,
      child: FocusableActionDetector(
        enabled: groupEnabled,
        onShowFocusHighlight: (v) => setState(() => _focus = v),
        shortcuts: const {
          SingleActivator(LogicalKeyboardKey.arrowLeft): _MoveIntent(-1),
          SingleActivator(LogicalKeyboardKey.arrowRight): _MoveIntent(1),
        },
        actions: {
          _MoveIntent: CallbackAction<_MoveIntent>(
            onInvoke: (i) => _move(i.delta),
          ),
        },
        child: FocusRing(
          visible: _focus,
          child: Container(
            height: kControlHeight,
            decoration: BoxDecoration(border: Border.all(color: t.ctrl)),
            child: Row(mainAxisSize: MainAxisSize.min, children: segments),
          ),
        ),
      ),
    );
  }
}

class _MoveIntent extends Intent {
  const _MoveIntent(this.delta);
  final int delta;
}

/// Seletor (DOC-01 §5): caixa de 28 px com o valor e "▾"; abre um menu com
/// as opções. Teclado: Enter, Espaço ou ↓ abrem; setas navegam; Esc fecha.
class MorsaSelect<T> extends StatefulWidget {
  const MorsaSelect({
    super.key,
    required this.options,
    required this.value,
    required this.onChanged,
    required this.label,
    this.width = 200,
  });

  final List<MorsaOption<T>> options;
  final T value;
  final ValueChanged<T>? onChanged;

  /// Nome do campo para leitores de tela.
  final String label;
  final double width;

  @override
  State<MorsaSelect<T>> createState() => _MorsaSelectState<T>();
}

class _MorsaSelectState<T> extends State<MorsaSelect<T>> {
  final _menu = MenuController();
  bool _focus = false;
  bool _hover = false;

  void _open() {
    if (widget.onChanged == null) return;
    _menu.isOpen ? _menu.close() : _menu.open();
  }

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final enabled = widget.onChanged != null;
    final current = widget.options
        .where((o) => o.value == widget.value)
        .firstOrNull;
    final fg = enabled ? t.ink : t.dis;
    final text = Theme.of(context).textTheme.bodyMedium!.copyWith(color: fg);

    return MenuAnchor(
      controller: _menu,
      style: morsaMenuStyle(context),
      alignmentOffset: const Offset(0, 2),
      menuChildren: [
        for (final o in widget.options)
          MorsaMenuItem(
            label: o.label,
            selected: o.value == widget.value,
            onPressed: o.enabled ? () => widget.onChanged?.call(o.value) : null,
          ),
      ],
      builder: (context, controller, _) => Semantics(
        button: true,
        enabled: enabled,
        focusable: enabled,
        focused: _focus,
        expanded: controller.isOpen,
        label: widget.label,
        value: current?.label,
        onTap: enabled ? _open : null,
        excludeSemantics: true,
        child: FocusableActionDetector(
          enabled: enabled,
          mouseCursor: enabled
              ? SystemMouseCursors.click
              : SystemMouseCursors.basic,
          onShowFocusHighlight: (v) => setState(() => _focus = v),
          onShowHoverHighlight: (v) => setState(() => _hover = v),
          shortcuts: const {
            SingleActivator(LogicalKeyboardKey.enter): ActivateIntent(),
            SingleActivator(LogicalKeyboardKey.space): ActivateIntent(),
            SingleActivator(LogicalKeyboardKey.arrowDown): ActivateIntent(),
          },
          actions: {
            ActivateIntent: CallbackAction<ActivateIntent>(
              onInvoke: (_) => _open(),
            ),
          },
          child: GestureDetector(
            behavior: HitTestBehavior.opaque,
            excludeFromSemantics: true,
            onTap: enabled ? _open : null,
            child: FocusRing(
              visible: _focus,
              child: Container(
                width: widget.width,
                height: kControlHeight,
                padding: const EdgeInsets.symmetric(horizontal: 8),
                decoration: BoxDecoration(
                  color: enabled && _hover ? t.sunk : null,
                  border: Border.all(color: enabled ? t.ctrl : t.dis),
                ),
                child: Row(
                  children: [
                    Expanded(
                      child: Text(
                        current?.label ?? '',
                        style: text,
                        overflow: TextOverflow.ellipsis,
                      ),
                    ),
                    Text('▾', style: TextStyle(fontSize: 10, color: fg)),
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

/// Estilo dos menus (seletor e menu de contexto): papel, régua de tinta,
/// cantos retos, sem sombra.
MenuStyle morsaMenuStyle(BuildContext context) {
  final t = context.tokens;
  return MenuStyle(
    backgroundColor: WidgetStatePropertyAll(t.paper),
    surfaceTintColor: const WidgetStatePropertyAll(Colors.transparent),
    shadowColor: const WidgetStatePropertyAll(Colors.transparent),
    elevation: const WidgetStatePropertyAll(0),
    padding: const WidgetStatePropertyAll(EdgeInsets.symmetric(vertical: 4)),
    shape: WidgetStatePropertyAll(
      RoundedRectangleBorder(side: BorderSide(color: t.ink)),
    ),
  );
}

/// Item de menu de 26 px (DOC-01 §5): o item com foco do teclado fica
/// invertido (tinta); hover é tom de papel; o atalho aparece à direita.
class MorsaMenuItem extends StatefulWidget {
  const MorsaMenuItem({
    super.key,
    required this.label,
    required this.onPressed,
    this.shortcut,
    this.selected = false,
  });

  final String label;
  final VoidCallback? onPressed;

  /// Atalho exibido à direita (ex.: `Ctrl D`, `Del`).
  final String? shortcut;

  /// Valor atual de um seletor: aparece em 500.
  final bool selected;

  @override
  State<MorsaMenuItem> createState() => _MorsaMenuItemState();
}

class _MorsaMenuItemState extends State<MorsaMenuItem> {
  final _states = WidgetStatesController();

  @override
  void dispose() {
    _states.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final base = Theme.of(context).textTheme.bodySmall!;
    final shortcut = widget.shortcut;
    return MenuItemButton(
      onPressed: widget.onPressed,
      statesController: _states,
      requestFocusOnHover: false,
      trailingIcon: shortcut == null
          ? null
          : ValueListenableBuilder(
              valueListenable: _states,
              builder: (context, states, _) => Text(
                shortcut,
                style: base.copyWith(
                  color: widget.onPressed == null
                      ? t.dis
                      : states.contains(WidgetState.focused)
                      ? t.paper
                      : t.ink2,
                ),
              ),
            ),
      style: ButtonStyle(
        minimumSize: const WidgetStatePropertyAll(Size(160, 26)),
        maximumSize: const WidgetStatePropertyAll(Size(double.infinity, 26)),
        padding: const WidgetStatePropertyAll(
          EdgeInsets.symmetric(horizontal: 10),
        ),
        shape: const WidgetStatePropertyAll(RoundedRectangleBorder()),
        overlayColor: const WidgetStatePropertyAll(Colors.transparent),
        splashFactory: NoSplash.splashFactory,
        textStyle: WidgetStatePropertyAll(
          base.copyWith(fontWeight: widget.selected ? FontWeight.w500 : null),
        ),
        backgroundColor: WidgetStateProperty.resolveWith((s) {
          if (s.contains(WidgetState.disabled)) return null;
          if (s.contains(WidgetState.focused)) return t.ink;
          if (s.contains(WidgetState.hovered)) return t.sunk;
          return null;
        }),
        foregroundColor: WidgetStateProperty.resolveWith((s) {
          if (s.contains(WidgetState.disabled)) return t.dis;
          if (s.contains(WidgetState.focused)) return t.paper;
          return t.ink;
        }),
      ),
      child: Text(widget.label),
    );
  }
}

/// Régua fraca entre grupos de itens de menu.
class MorsaMenuDivider extends StatelessWidget {
  const MorsaMenuDivider({super.key});

  @override
  Widget build(BuildContext context) => Container(
    height: 1,
    margin: const EdgeInsets.symmetric(vertical: 4),
    color: context.tokens.weak,
  );
}
