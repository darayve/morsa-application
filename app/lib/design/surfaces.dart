import 'package:flutter/material.dart';

import 'focus_ring.dart';
import 'theme.dart';

/// Pauta: linhas horizontais de 1 px em `weak` a cada [step] px (§9.5).
/// Fundo das áreas de soltar (24 px) e das miniaturas (12 px).
class _RuledPainter extends CustomPainter {
  _RuledPainter(this.color, this.step);

  final Color color;
  final double step;

  @override
  void paint(Canvas canvas, Size size) {
    final paint = Paint()..color = color;
    for (var y = step - 1; y < size.height; y += step) {
      canvas.drawRect(Rect.fromLTWH(0, y, size.width, 1), paint);
    }
  }

  @override
  bool shouldRepaint(_RuledPainter old) =>
      old.color != color || old.step != step;
}

/// Borda tracejada de 1 px: só em áreas de soltar (§9.5).
class _DashedBorderPainter extends CustomPainter {
  _DashedBorderPainter(this.color);

  final Color color;

  @override
  void paint(Canvas canvas, Size size) {
    const dash = 4.0;
    const gap = 3.0;
    final paint = Paint()..color = color;
    for (var x = 0.0; x < size.width; x += dash + gap) {
      final w = (x + dash).clamp(0, size.width) - x;
      canvas
        ..drawRect(Rect.fromLTWH(x, 0, w, 1), paint)
        ..drawRect(Rect.fromLTWH(x, size.height - 1, w, 1), paint);
    }
    for (var y = 0.0; y < size.height; y += dash + gap) {
      final h = (y + dash).clamp(0, size.height) - y;
      canvas
        ..drawRect(Rect.fromLTWH(0, y, 1, h), paint)
        ..drawRect(Rect.fromLTWH(size.width - 1, y, 1, h), paint);
    }
  }

  @override
  bool shouldRepaint(_DashedBorderPainter old) => old.color != color;
}

/// Área de soltar (DOC-01 §5): contorno tracejado, pauta de 24 px e os
/// textos sobre faixas de papel. Com `active` (arquivo sendo arrastado por
/// cima), o fundo fica em `sunk`.
///
/// Este é só o desenho; receber os arquivos é da tela (Marco 3).
class MorsaDropArea extends StatelessWidget {
  const MorsaDropArea({
    super.key,
    required this.title,
    required this.orLabel,
    required this.action,
    required this.formats,
    this.active = false,
    this.height = 168,
  });

  final String title;
  final String orLabel;
  final Widget action;
  final String formats;
  final bool active;
  final double height;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    Widget onPaper(Widget child) => ColoredBox(
      color: t.paper,
      child: Padding(
        padding: const EdgeInsets.symmetric(horizontal: 6),
        child: child,
      ),
    );
    return CustomPaint(
      foregroundPainter: _DashedBorderPainter(t.ink),
      child: CustomPaint(
        painter: _RuledPainter(t.weak, 24),
        child: Container(
          width: double.infinity,
          height: height,
          color: active ? t.sunk.withValues(alpha: 0.6) : null,
          child: Column(
            mainAxisAlignment: MainAxisAlignment.center,
            children: [
              onPaper(
                Text(
                  title,
                  textAlign: TextAlign.center,
                  style: Theme.of(context).textTheme.labelLarge,
                ),
              ),
              const SizedBox(height: 12),
              onPaper(
                Row(
                  mainAxisSize: MainAxisSize.min,
                  children: [
                    Text(
                      orLabel,
                      style: TextStyle(fontSize: 11, color: t.ink2),
                    ),
                    const SizedBox(width: 12),
                    action,
                  ],
                ),
              ),
              const SizedBox(height: 12),
              onPaper(
                Text(
                  formats,
                  textAlign: TextAlign.center,
                  style: TextStyle(fontSize: 11, color: t.ink2),
                ),
              ),
            ],
          ),
        ),
      ),
    );
  }
}

/// Miniatura de página (DOC-01 §5): 84 × 112 px, pauta de 12 px e faixa de
/// 6 px na cor da categoria da origem. Selecionada: contorno de 2 px. Foco:
/// anel azul por fora. Embaixo, a referência (A1, B6…) e um detalhe opcional
/// (☒, ↻ 90°).
class MorsaThumbnail extends StatefulWidget {
  const MorsaThumbnail({
    super.key,
    required this.reference,
    required this.categoryColor,
    this.detail,
    this.selected = false,
    this.onTap,
    this.image,
    this.semanticLabel,
  });

  final String reference;
  final Color categoryColor;
  final String? detail;
  final bool selected;
  final VoidCallback? onTap;

  /// Imagem da página, quando já renderizada. Sem ela, só a pauta.
  final Widget? image;
  final String? semanticLabel;

  @override
  State<MorsaThumbnail> createState() => _MorsaThumbnailState();
}

class _MorsaThumbnailState extends State<MorsaThumbnail> {
  bool _focus = false;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    return Semantics(
      button: widget.onTap != null,
      selected: widget.selected,
      label: widget.semanticLabel ?? widget.reference,
      onTap: widget.onTap,
      excludeSemantics: true,
      child: FocusableActionDetector(
        enabled: widget.onTap != null,
        onShowFocusHighlight: (v) => setState(() => _focus = v),
        actions: {
          ActivateIntent: CallbackAction<ActivateIntent>(
            onInvoke: (_) => widget.onTap?.call(),
          ),
        },
        child: GestureDetector(
          behavior: HitTestBehavior.opaque,
          excludeFromSemantics: true,
          onTap: widget.onTap,
          child: SizedBox(
            width: 84,
            child: Column(
              crossAxisAlignment: CrossAxisAlignment.stretch,
              mainAxisSize: MainAxisSize.min,
              children: [
                FocusRing(
                  visible: _focus,
                  child: CustomPaint(
                    painter: widget.image == null
                        ? _RuledPainter(t.weak, 12)
                        : null,
                    child: Container(
                      height: 112,
                      decoration: BoxDecoration(
                        border: Border.all(
                          color: t.ink,
                          width: widget.selected ? 2 : 1,
                        ),
                      ),
                      child: Row(
                        crossAxisAlignment: CrossAxisAlignment.stretch,
                        children: [
                          Container(width: 6, color: widget.categoryColor),
                          Expanded(child: widget.image ?? const SizedBox()),
                        ],
                      ),
                    ),
                  ),
                ),
                const SizedBox(height: 4),
                Row(
                  children: [
                    Text(
                      widget.reference,
                      style: TextStyle(
                        fontSize: 11,
                        fontWeight: FontWeight.w700,
                        color: t.ink,
                      ),
                    ),
                    const Spacer(),
                    if (widget.detail != null)
                      Text(
                        widget.detail!,
                        style: TextStyle(fontSize: 11, color: t.ink2),
                      ),
                  ],
                ),
              ],
            ),
          ),
        ),
      ),
    );
  }
}

/// Régua azul de 2 px que marca o ponto de inserção ao arrastar miniaturas.
class InsertionMarker extends StatelessWidget {
  const InsertionMarker({super.key, this.height = 112});

  final double height;

  @override
  Widget build(BuildContext context) =>
      Container(width: 2, height: height, color: context.tokens.focus);
}
