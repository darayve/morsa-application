import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'focus_ring.dart';
import 'theme.dart';

/// Altura padrão de controles (§9.5).
const kControlHeight = 28.0;

/// Botão do sistema visual: cantos retos, 28 px, anel de foco azul por fora.
/// Primário é tinta cheia; secundário é contorno. Hover é tom de papel,
/// nunca cor (DOC-01 §5).
class MorsaButton extends StatefulWidget {
  const MorsaButton({
    super.key,
    required this.label,
    required this.onPressed,
    this.primary = false,
    this.shortcut,
    this.autofocus = false,
    this.focusNode,
  });

  final String label;
  final VoidCallback? onPressed;
  final bool primary;

  /// Atalho exibido no próprio controle (ex.: `Ctrl O`).
  final String? shortcut;
  final bool autofocus;
  final FocusNode? focusNode;

  @override
  State<MorsaButton> createState() => _MorsaButtonState();
}

class _MorsaButtonState extends State<MorsaButton> {
  bool _hover = false;
  bool _focus = false;
  bool _pressed = false;

  bool get _enabled => widget.onPressed != null;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final Color bg;
    final Color fg;
    final Color border;
    if (!_enabled) {
      bg = widget.primary ? t.dis : Colors.transparent;
      fg = widget.primary ? t.paper : t.dis;
      border = t.dis;
    } else if (widget.primary) {
      bg = _pressed ? t.inkP : (_hover ? t.inkH : t.ink);
      fg = t.paper;
      border = bg;
    } else {
      bg = _pressed ? t.sel : (_hover ? t.sunk : Colors.transparent);
      fg = t.ink;
      border = t.ctrl;
    }

    final text = Theme.of(context).textTheme.labelLarge!.copyWith(color: fg);

    return Semantics(
      button: true,
      enabled: _enabled,
      focusable: _enabled,
      focused: _focus,
      label: widget.label,
      // Leitores de tela (e o teste web) acionam o botão por aqui.
      onTap: widget.onPressed,
      excludeSemantics: true,
      child: _Pressable(
        enabled: _enabled,
        autofocus: widget.autofocus,
        focusNode: widget.focusNode,
        onPressed: widget.onPressed,
        onHover: (v) => setState(() => _hover = v),
        onFocus: (v) => setState(() => _focus = v),
        onPressedChange: (v) => setState(() => _pressed = v),
        child: FocusRing(
          visible: _focus,
          child: Container(
            height: kControlHeight,
            padding: EdgeInsets.only(
              left: 12,
              right: widget.shortcut == null ? 12 : 6,
            ),
            decoration: BoxDecoration(
              color: bg,
              border: Border.all(color: border),
            ),
            child: Row(
              mainAxisSize: MainAxisSize.min,
              children: [
                Text(widget.label, style: text),
                if (widget.shortcut != null) ...[
                  const SizedBox(width: 10),
                  ShortcutChip(
                    widget.shortcut!,
                    inverted: widget.primary && _enabled,
                  ),
                ],
              ],
            ),
          ),
        ),
      ),
    );
  }
}

/// Link de texto sublinhado (DOC-01 §5): hover engrossa o sublinhado,
/// pressionado fica em `ink2`, foco é o anel azul.
class MorsaLink extends StatefulWidget {
  const MorsaLink({super.key, required this.label, required this.onPressed});

  final String label;
  final VoidCallback? onPressed;

  @override
  State<MorsaLink> createState() => _MorsaLinkState();
}

class _MorsaLinkState extends State<MorsaLink> {
  bool _hover = false;
  bool _focus = false;
  bool _pressed = false;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final enabled = widget.onPressed != null;
    final color = !enabled ? t.dis : (_pressed ? t.ink2 : t.ink);
    return Semantics(
      link: true,
      enabled: enabled,
      focusable: enabled,
      focused: _focus,
      label: widget.label,
      onTap: widget.onPressed,
      excludeSemantics: true,
      child: _Pressable(
        enabled: enabled,
        onPressed: widget.onPressed,
        onHover: (v) => setState(() => _hover = v),
        onFocus: (v) => setState(() => _focus = v),
        onPressedChange: (v) => setState(() => _pressed = v),
        child: FocusRing(
          visible: _focus,
          child: Container(
            // O Flutter não tem `text-underline-offset`; o sublinhado é uma
            // régua 3 px abaixo da linha de base.
            padding: const EdgeInsets.only(bottom: 1),
            decoration: BoxDecoration(
              border: Border(
                bottom: BorderSide(
                  color: color,
                  width: enabled && (_hover || _pressed) ? 2 : 1,
                ),
              ),
            ),
            child: Text(
              widget.label,
              style: Theme.of(context).textTheme.bodyMedium!
                  .copyWith(color: color),
            ),
          ),
        ),
      ),
    );
  }
}

/// Clique, Enter e Espaço acionam; informa hover, foco e pressão para o
/// componente desenhar seus estados. Base de botões, links e afins.
class _Pressable extends StatelessWidget {
  const _Pressable({
    required this.enabled,
    required this.onPressed,
    required this.onHover,
    required this.onFocus,
    required this.onPressedChange,
    required this.child,
    this.autofocus = false,
    this.focusNode,
  });

  final bool enabled;
  final VoidCallback? onPressed;
  final ValueChanged<bool> onHover;
  final ValueChanged<bool> onFocus;
  final ValueChanged<bool> onPressedChange;
  final bool autofocus;
  final FocusNode? focusNode;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    return FocusableActionDetector(
      enabled: enabled,
      autofocus: autofocus,
      focusNode: focusNode,
      mouseCursor: enabled
          ? SystemMouseCursors.click
          : SystemMouseCursors.basic,
      onShowHoverHighlight: onHover,
      onShowFocusHighlight: onFocus,
      shortcuts: const {
        SingleActivator(LogicalKeyboardKey.enter): ActivateIntent(),
        SingleActivator(LogicalKeyboardKey.numpadEnter): ActivateIntent(),
        SingleActivator(LogicalKeyboardKey.space): ActivateIntent(),
      },
      actions: {
        ActivateIntent: CallbackAction<ActivateIntent>(
          onInvoke: (_) => onPressed?.call(),
        ),
      },
      child: GestureDetector(
        behavior: HitTestBehavior.opaque,
        excludeFromSemantics: true,
        onTapDown: enabled ? (_) => onPressedChange(true) : null,
        onTapCancel: enabled ? () => onPressedChange(false) : null,
        onTapUp: enabled
            ? (_) {
                onPressedChange(false);
                onPressed?.call();
              }
            : null,
        child: child,
      ),
    );
  }
}

/// Atalho visível no controle: 11 px, borda de régua fraca (§9.7).
class ShortcutChip extends StatelessWidget {
  const ShortcutChip(this.text, {super.key, this.inverted = false});

  final String text;
  final bool inverted;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final color = inverted ? t.paper : t.ink2;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 4),
      decoration: BoxDecoration(
        border: Border.all(
          color: inverted ? t.paper.withValues(alpha: 0.5) : t.weak2,
        ),
      ),
      child: Text(
        text,
        style: TextStyle(
          fontSize: 11,
          fontWeight: FontWeight.w400,
          color: color,
          height: 16 / 11,
        ),
      ),
    );
  }
}

/// Rótulo de seção com régua forte de 2 px embaixo (§9.5).
class SectionLabel extends StatelessWidget {
  const SectionLabel(this.text, {super.key, this.trailing});

  final String text;
  final Widget? trailing;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    return Container(
      height: kControlHeight,
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: t.ink, width: 2)),
      ),
      child: Row(
        children: [
          Expanded(
            child: Semantics(
              header: true,
              child: Text(
                text.toUpperCase(),
                style: Theme.of(context).textTheme.labelMedium,
              ),
            ),
          ),
          ?trailing,
        ],
      ),
    );
  }
}

/// Linha de ficha rótulo → valor: 26 px, rótulo de 130 px em `ink2`, valor
/// em 500, régua fraca embaixo. A última linha da ficha usa régua de tinta.
class FichaRow extends StatelessWidget {
  const FichaRow({
    super.key,
    required this.label,
    required this.value,
    this.last = false,
  });

  final String label;
  final Widget value;
  final bool last;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final style = Theme.of(context).textTheme.bodySmall!;
    return Container(
      constraints: const BoxConstraints(minHeight: 26),
      padding: const EdgeInsets.symmetric(vertical: 4),
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: last ? t.ink : t.weak)),
      ),
      child: Row(
        children: [
          SizedBox(
            width: 130,
            child: Text(label, style: style.copyWith(color: t.ink2)),
          ),
          Expanded(
            child: DefaultTextStyle(
              style: style.copyWith(fontWeight: FontWeight.w500),
              child: value,
            ),
          ),
        ],
      ),
    );
  }
}

/// Campo de texto de 28 px, cantos retos (DOC-01 §5).
///
/// Estados: normal (borda de controle), foco (anel azul por fora), erro
/// (borda de 2 px em `fail` e mensagem "✕ …" embaixo) e desabilitado.
/// Campos numéricos alinham à direita, com a unidade na mesma caixa (§9.3).
class MorsaTextField extends StatefulWidget {
  const MorsaTextField({
    super.key,
    required this.controller,
    required this.label,
    this.errorText,
    this.width = 96,
    this.suffix,
    this.numeric = false,
    this.enabled = true,
    this.keyboardType,
    this.inputFormatters,
    this.onChanged,
  });

  final TextEditingController controller;

  /// Nome do campo para leitores de tela.
  final String label;
  final String? errorText;
  final double width;

  /// Unidade exibida dentro da caixa, à direita (ex.: `DPI`).
  final String? suffix;
  final bool numeric;
  final bool enabled;
  final TextInputType? keyboardType;
  final List<TextInputFormatter>? inputFormatters;
  final ValueChanged<String>? onChanged;

  @override
  State<MorsaTextField> createState() => _MorsaTextFieldState();
}

class _MorsaTextFieldState extends State<MorsaTextField> {
  final _focus = FocusNode();

  @override
  void initState() {
    super.initState();
    _focus.addListener(() => setState(() {}));
  }

  @override
  void dispose() {
    _focus.dispose();
    super.dispose();
  }

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final body = Theme.of(context).textTheme.bodyMedium!;
    final error = widget.errorText;
    final enabled = widget.enabled;
    final border = !enabled
        ? Border.all(color: t.dis)
        : error != null
        ? Border.all(color: t.fail, width: 2)
        : Border.all(color: t.ctrl);

    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      mainAxisSize: MainAxisSize.min,
      children: [
        FocusRing(
          visible: _focus.hasFocus,
          child: Container(
            width: widget.width,
            height: kControlHeight,
            padding: const EdgeInsets.symmetric(horizontal: 8),
            decoration: BoxDecoration(border: border),
            child: Row(
              children: [
                Expanded(
                  child: Semantics(
                    label: widget.label,
                    child: TextField(
                      controller: widget.controller,
                      focusNode: _focus,
                      enabled: enabled,
                      style: body.copyWith(color: enabled ? t.ink : t.dis),
                      keyboardType: widget.keyboardType,
                      inputFormatters: widget.inputFormatters,
                      onChanged: widget.onChanged,
                      textAlign: widget.numeric
                          ? TextAlign.right
                          : TextAlign.start,
                      decoration: const InputDecoration(
                        isCollapsed: true,
                        border: InputBorder.none,
                      ),
                    ),
                  ),
                ),
                if (widget.suffix != null) ...[
                  const SizedBox(width: 6),
                  Text(
                    widget.suffix!,
                    style: TextStyle(fontSize: 11, color: t.ink2),
                  ),
                ],
              ],
            ),
          ),
        ),
        if (error != null)
          Padding(
            padding: const EdgeInsets.only(top: 4),
            child: Semantics(
              liveRegion: true,
              child: Text(
                '✕ $error',
                style: TextStyle(
                  fontSize: 11,
                  fontWeight: FontWeight.w500,
                  color: t.fail,
                ),
              ),
            ),
          ),
      ],
    );
  }
}
