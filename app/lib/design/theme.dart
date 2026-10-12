import 'package:flutter/material.dart';

import 'tokens.dart';

/// Famílias empacotadas (§9.3). Nada vem da internet.
const kMono = 'Ioskeley Mono NL';
const kTitle = 'IBM Plex Sans Condensed';

/// Fallback se uma fonte faltar: as monoespaçadas do sistema, nunca Roboto.
const kFallback = ['monospace'];

extension MorsaContext on BuildContext {
  MorsaTokens get tokens => Theme.of(this).extension<MorsaTokens>()!;
}

/// Tema claro ou escuro montado a partir dos tokens.
ThemeData buildTheme(Brightness brightness) {
  final t = brightness == Brightness.light
      ? MorsaTokens.light
      : MorsaTokens.dark;

  TextStyle mono(double size, FontWeight weight, [Color? color]) => TextStyle(
    fontFamily: kMono,
    fontFamilyFallback: kFallback,
    fontSize: size,
    fontWeight: weight,
    color: color ?? t.ink,
    height: 1.4,
  );

  return ThemeData(
    brightness: brightness,
    useMaterial3: true,
    fontFamily: kMono,
    fontFamilyFallback: kFallback,
    scaffoldBackgroundColor: t.paper,
    // Sem ondas, sem sombras, sem cor de marca (§9.1).
    splashFactory: NoSplash.splashFactory,
    highlightColor: Colors.transparent,
    hoverColor: t.sunk,
    focusColor: Colors.transparent,
    colorScheme: ColorScheme(
      brightness: brightness,
      primary: t.ink,
      onPrimary: t.paper,
      secondary: t.ink2,
      onSecondary: t.paper,
      error: t.fail,
      onError: t.paper,
      surface: t.paper,
      onSurface: t.ink,
    ),
    textSelectionTheme: TextSelectionThemeData(
      cursorColor: t.ink,
      selectionColor: t.sel,
    ),
    textTheme: TextTheme(
      // Título de tela: Plex Condensed 28–32 / 700.
      headlineMedium: TextStyle(
        fontFamily: kTitle,
        fontSize: 30,
        fontWeight: FontWeight.w700,
        letterSpacing: -0.3,
        color: t.ink,
        height: 1.15,
      ),
      // Rótulo de seção: Plex Condensed 12 / 700, caixa alta (§9.3).
      labelMedium: TextStyle(
        fontFamily: kTitle,
        fontSize: 12,
        fontWeight: FontWeight.w700,
        letterSpacing: 0.9,
        color: t.ink,
      ),
      titleMedium: mono(16, FontWeight.w700),
      bodyMedium: mono(13, FontWeight.w400),
      labelLarge: mono(13, FontWeight.w500),
      bodySmall: mono(12, FontWeight.w400),
      labelSmall: mono(11, FontWeight.w300, t.ink2),
    ),
    extensions: [t],
  );
}
