import 'package:flutter/material.dart';

import 'tokens.dart';

/// Famílias empacotadas (§9.3). Nada vem da internet.
const kMono = 'Ioskeley Mono NL';
const kTitle = 'IBM Plex Sans Condensed';

/// Escala de texto e alturas. Maior que a dos protótipos (13 px no corpo),
/// por pedido de leitura mais confortável; tudo passa por aqui.
const kFsMeta = 14.0; // metadado, atalho, unidade (era 11)
const kFsSmall = 15.0; // ficha e tabela (era 12)
const kFsBody = 16.0; // corpo, células, campos, botões (era 13)
const kFsPanel = 19.0; // título de painel (era 16)
const kFsTitle = 34.0; // título de tela (era 30)

/// Altura de controles, linhas de tabela, ficha e barra de status.
const kControlHeight = 34.0;
const kFichaHeight = 32.0;
const kStatusBarHeight = 30.0;

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
        fontSize: kFsTitle,
        fontWeight: FontWeight.w700,
        letterSpacing: -0.3,
        color: t.ink,
        height: 1.15,
      ),
      // Rótulo de seção: Plex Condensed 12 / 700, caixa alta (§9.3).
      labelMedium: TextStyle(
        fontFamily: kTitle,
        fontSize: kFsSmall,
        fontWeight: FontWeight.w700,
        letterSpacing: 0.9,
        color: t.ink,
      ),
      titleMedium: mono(kFsPanel, FontWeight.w700),
      bodyMedium: mono(kFsBody, FontWeight.w400),
      labelLarge: mono(kFsBody, FontWeight.w500),
      bodySmall: mono(kFsSmall, FontWeight.w400),
      labelSmall: mono(kFsMeta, FontWeight.w300, t.ink2),
    ),
    extensions: [t],
  );
}
