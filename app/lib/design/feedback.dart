import 'package:flutter/material.dart';

import 'theme.dart';
import 'tokens.dart';

/// Situação de um item ou ordem. Sempre aparece com texto, nunca só com cor
/// (RF09, DOC-01 §2).
enum ItemStatus { queued, running, done, failed }

/// Cor de cada situação. Todas passam 4,5:1 sobre o papel nos dois temas.
Color statusColor(MorsaTokens t, ItemStatus s) => switch (s) {
  ItemStatus.queued => t.ink2,
  ItemStatus.running => t.run,
  ItemStatus.done => t.ok,
  ItemStatus.failed => t.fail,
};

/// Texto da situação na cor dela ("Executando", "Falha · 31%"…).
class StatusText extends StatelessWidget {
  const StatusText(this.status, this.text, {super.key});

  final ItemStatus status;
  final String text;

  @override
  Widget build(BuildContext context) {
    return Text(
      text,
      style: Theme.of(context).textTheme.bodySmall!.copyWith(
        color: statusColor(context.tokens, status),
        fontWeight: status == ItemStatus.queued
            ? FontWeight.w400
            : FontWeight.w500,
      ),
    );
  }
}

/// Barra de progresso de contorno com preenchimento em tinta (DOC-01 §5).
/// 8 px para um item; 14 px para a ordem inteira (`large`).
class MorsaProgressBar extends StatelessWidget {
  const MorsaProgressBar({
    super.key,
    required this.value,
    this.large = false,
    this.semanticLabel,
  });

  /// 0–1. Nulo ou zero desenha a barra vazia (na fila).
  final double? value;
  final bool large;
  final String? semanticLabel;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final v = (value ?? 0).clamp(0.0, 1.0);
    return Semantics(
      label: semanticLabel,
      value: '${(v * 100).round()}%',
      child: Container(
        width: double.infinity,
        height: large ? 14 : 8,
        decoration: BoxDecoration(border: Border.all(color: t.ink)),
        child: FractionallySizedBox(
          alignment: Alignment.centerLeft,
          widthFactor: v,
          child: ColoredBox(color: t.ink),
        ),
      ),
    );
  }
}

/// Tipo de aviso: atenção (contorno de tinta) ou falha (contorno vermelho).
enum NoticeKind { attention, failure }

/// Aviso em caixa (DOC-01 §5): faixa de título de 26 px em caixa alta com
/// quadrado de 8 px, e o texto embaixo.
class MorsaNotice extends StatelessWidget {
  const MorsaNotice({
    super.key,
    required this.kind,
    required this.title,
    required this.body,
  });

  final NoticeKind kind;
  final String title;
  final String body;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final color = kind == NoticeKind.failure ? t.fail : t.ink;
    return Semantics(
      container: true,
      liveRegion: kind == NoticeKind.failure,
      child: Container(
        decoration: BoxDecoration(border: Border.all(color: color)),
        child: Column(
          crossAxisAlignment: CrossAxisAlignment.stretch,
          mainAxisSize: MainAxisSize.min,
          children: [
            Container(
              height: 26,
              padding: const EdgeInsets.symmetric(horizontal: 10),
              decoration: BoxDecoration(
                border: Border(bottom: BorderSide(color: color)),
              ),
              child: Row(
                children: [
                  Container(width: 8, height: 8, color: color),
                  const SizedBox(width: 8),
                  Expanded(
                    child: Text(
                      title.toUpperCase(),
                      overflow: TextOverflow.ellipsis,
                      style: TextStyle(
                        fontSize: 11,
                        fontWeight: FontWeight.w700,
                        letterSpacing: 0.66,
                        color: color,
                      ),
                    ),
                  ),
                ],
              ),
            ),
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 10, vertical: 8),
              child: Text(body, style: Theme.of(context).textTheme.bodySmall),
            ),
          ],
        ),
      ),
    );
  }
}
