import 'package:flutter/widgets.dart';

import 'theme.dart';

/// Anel de foco azul de 2 px (§9.7).
///
/// Em controles, fica por fora, com respiro de 2 px, desenhado sobre o
/// layout (como o `outline` do CSS): não empurra nada. Em linhas de tabela
/// e listas, fica por dentro (`inset`), para não ser cortado pela rolagem.
class FocusRing extends StatelessWidget {
  const FocusRing({
    super.key,
    required this.visible,
    required this.child,
    this.inset = false,
  });

  final bool visible;
  final bool inset;
  final Widget child;

  @override
  Widget build(BuildContext context) {
    final ring = BoxDecoration(
      border: Border.all(color: context.tokens.focus, width: 2),
    );
    if (inset) {
      return DecoratedBox(
        position: DecorationPosition.foreground,
        decoration: visible ? ring : const BoxDecoration(),
        child: child,
      );
    }
    return Stack(
      clipBehavior: Clip.none,
      children: [
        child,
        if (visible)
          Positioned(
            left: -4,
            top: -4,
            right: -4,
            bottom: -4,
            child: IgnorePointer(child: DecoratedBox(decoration: ring)),
          ),
      ],
    );
  }
}
