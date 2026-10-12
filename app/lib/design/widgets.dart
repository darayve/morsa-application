import 'package:flutter/material.dart';
import 'package:flutter/services.dart';

import 'theme.dart';

/// Altura padrão de controles (§9.5).
const kControlHeight = 28.0;

/// Botão do sistema visual: cantos retos, 28 px, anel de foco azul de 2 px
/// com respiro de 2 px por fora. Primário é tinta cheia; secundário é contorno.
class MorsaButton extends StatefulWidget {
  const MorsaButton({
    super.key,
    required this.label,
    required this.onPressed,
    this.primary = false,
    this.shortcut,
    this.autofocus = false,
  });

  final String label;
  final VoidCallback? onPressed;
  final bool primary;

  /// Atalho exibido no próprio controle (ex.: `Ctrl O`).
  final String? shortcut;
  final bool autofocus;

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
      label: widget.label,
      excludeSemantics: true,
      child: FocusableActionDetector(
        enabled: _enabled,
        autofocus: widget.autofocus,
        mouseCursor: _enabled
            ? SystemMouseCursors.click
            : SystemMouseCursors.basic,
        onShowHoverHighlight: (v) => setState(() => _hover = v),
        onShowFocusHighlight: (v) => setState(() => _focus = v),
        shortcuts: const {
          SingleActivator(LogicalKeyboardKey.enter): ActivateIntent(),
          SingleActivator(LogicalKeyboardKey.space): ActivateIntent(),
        },
        actions: {
          ActivateIntent: CallbackAction<ActivateIntent>(
            onInvoke: (_) => widget.onPressed?.call(),
          ),
        },
        child: GestureDetector(
          onTapDown: _enabled ? (_) => setState(() => _pressed = true) : null,
          onTapCancel: () => setState(() => _pressed = false),
          onTapUp: _enabled
              ? (_) {
                  setState(() => _pressed = false);
                  widget.onPressed?.call();
                }
              : null,
          child: Container(
            // Respiro de 2 px + anel de 2 px por fora do controle.
            padding: const EdgeInsets.all(2),
            decoration: BoxDecoration(
              border: Border.all(
                color: _focus ? t.focus : Colors.transparent,
                width: 2,
              ),
            ),
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
      padding: const EdgeInsets.only(bottom: 6),
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

/// Linha de ficha rótulo → valor, 26 px, com régua fraca embaixo.
class FichaRow extends StatelessWidget {
  const FichaRow({super.key, required this.label, required this.value});

  final String label;
  final Widget value;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final style = Theme.of(context).textTheme.bodySmall!;
    return Container(
      constraints: const BoxConstraints(minHeight: 26),
      padding: const EdgeInsets.symmetric(vertical: 4),
      decoration: BoxDecoration(
        border: Border(bottom: BorderSide(color: t.weak)),
      ),
      child: Row(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SizedBox(
            width: 110,
            child: Text(label, style: style.copyWith(color: t.ink2)),
          ),
          Expanded(
            child: DefaultTextStyle(style: style, child: value),
          ),
        ],
      ),
    );
  }
}

/// Campo de texto de 28 px, cantos retos; borda vermelha e mensagem no erro.
class MorsaTextField extends StatelessWidget {
  const MorsaTextField({
    super.key,
    required this.controller,
    required this.label,
    this.errorText,
    this.width = 96,
    this.suffix,
    this.keyboardType,
    this.inputFormatters,
    this.onChanged,
  });

  final TextEditingController controller;
  final String label;
  final String? errorText;
  final double width;
  final String? suffix;
  final TextInputType? keyboardType;
  final List<TextInputFormatter>? inputFormatters;
  final ValueChanged<String>? onChanged;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final body = Theme.of(context).textTheme.bodyMedium!;
    OutlineInputBorder border(Color c, [double w = 1]) => OutlineInputBorder(
      borderRadius: BorderRadius.zero,
      borderSide: BorderSide(color: c, width: w),
    );
    return Column(
      crossAxisAlignment: CrossAxisAlignment.start,
      children: [
        SizedBox(
          width: width,
          height: kControlHeight,
          child: TextField(
            controller: controller,
            style: body,
            keyboardType: keyboardType,
            inputFormatters: inputFormatters,
            onChanged: onChanged,
            textAlign: TextAlign.right, // números à direita (§9.3)
            decoration: InputDecoration(
              isDense: true,
              contentPadding: const EdgeInsets.symmetric(
                horizontal: 8,
                vertical: 6,
              ),
              suffixText: suffix,
              suffixStyle: body.copyWith(color: t.ink2),
              enabledBorder: border(errorText != null ? t.fail : t.ctrl),
              focusedBorder: border(errorText != null ? t.fail : t.focus, 2),
              semanticCounterText: label,
            ),
          ),
        ),
        if (errorText != null)
          Padding(
            padding: const EdgeInsets.only(top: 4),
            child: Text(
              '✕ ${errorText!}',
              style: Theme.of(context).textTheme.bodySmall!
                  .copyWith(color: t.fail),
            ),
          ),
      ],
    );
  }
}
