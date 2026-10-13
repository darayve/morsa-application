import 'package:flutter/material.dart';

import '../app/l10n/app_localizations.dart';
import '../app/version.dart';
import 'theme.dart';

/// Barra de status fixa no rodapé, com 24 px (§9.9).
/// A afirmação "Sem acesso à rede" é fixa, sem contador de bytes (A2).
class MorsaStatusBar extends StatelessWidget {
  const MorsaStatusBar({super.key, this.progress});

  /// Progresso resumido no centro ("Convertendo · 1 de 1"), quando houver.
  final String? progress;

  @override
  Widget build(BuildContext context) {
    final t = context.tokens;
    final l = AppLocalizations.of(context);
    final style = Theme.of(context).textTheme.labelSmall!
        .copyWith(fontWeight: FontWeight.w400);
    return Container(
      height: kStatusBarHeight,
      padding: const EdgeInsets.symmetric(horizontal: 12),
      decoration: BoxDecoration(
        color: t.paper,
        border: Border(top: BorderSide(color: t.ink)),
      ),
      child: Row(
        children: [
          // Esquerda: a afirmação fixa ocupa o espaço livre e corta com "…"
          // em janelas estreitas, sem empurrar a versão para fora.
          Expanded(
            child: Row(
              children: [
                Text('●', style: style.copyWith(color: t.ok, fontSize: 11)),
                const SizedBox(width: 6),
                Text(
                  l.statusOffline,
                  style: style.copyWith(
                    color: t.ink,
                    fontWeight: FontWeight.w500,
                  ),
                ),
                Flexible(
                  child: Text(
                    ' · ${l.statusNoNetwork} · ${l.statusLocal}',
                    style: style,
                    overflow: TextOverflow.ellipsis,
                  ),
                ),
              ],
            ),
          ),
          if (progress != null)
            Padding(
              padding: const EdgeInsets.symmetric(horizontal: 16),
              child: Semantics(
                liveRegion: true,
                child: Text(progress!, style: style.copyWith(color: t.run)),
              ),
            ),
          // Direita: nome e versão do app.
          Text(l.statusVersion(kAppVersion), style: style),
        ],
      ),
    );
  }
}
