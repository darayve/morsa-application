// GERADO por scripts/gen_tokens.dart a partir dos requisitos §9.4.
// Não edite à mão: altere o script e rode `dart run scripts/gen_tokens.dart`.

import 'package:flutter/material.dart';

/// Tokens de cor do sistema visual, nos temas claro e escuro.
@immutable
class MorsaTokens extends ThemeExtension<MorsaTokens> {
  const MorsaTokens({
    required this.paper,
    required this.ink,
    required this.ink2,
    required this.weak,
    required this.weak2,
    required this.ctrl,
    required this.sunk,
    required this.sel,
    required this.inkH,
    required this.inkP,
    required this.dis,
    required this.ok,
    required this.run,
    required this.fail,
    required this.focus,
    required this.img,
    required this.pdf,
    required this.zip,
    required this.fut,
  });

  /// Fundo.
  final Color paper;

  /// Texto e régua forte.
  final Color ink;

  /// Texto secundário.
  final Color ink2;

  /// Régua fraca.
  final Color weak;

  /// Régua fraca 2, pauta, borda de atalho.
  final Color weak2;

  /// Borda de controle.
  final Color ctrl;

  /// Hover.
  final Color sunk;

  /// Linha selecionada.
  final Color sel;

  /// Botão primário em hover.
  final Color inkH;

  /// Botão primário pressionado.
  final Color inkP;

  /// Desabilitado.
  final Color dis;

  /// Concluído.
  final Color ok;

  /// Executando.
  final Color run;

  /// Falha.
  final Color fail;

  /// Anel de foco.
  final Color focus;

  /// Etiqueta Imagens.
  final Color img;

  /// Etiqueta PDF.
  final Color pdf;

  /// Etiqueta Compactados.
  final Color zip;

  /// Etiqueta Em breve.
  final Color fut;

  static const light = MorsaTokens(
    paper: Color(0xFFFAFAF8), // #fafaf8
    ink: Color(0xFF121212), // #121212
    ink2: Color(0xFF5D5D58), // #5d5d58
    weak: Color(0xFFDCDCD7), // #dcdcd7
    weak2: Color(0xFFCFCFCA), // #cfcfca
    ctrl: Color(0xFF121212), // #121212
    sunk: Color(0xFFF0F0EC), // #f0f0ec
    sel: Color(0xFFE6E6E1), // #e6e6e1
    inkH: Color(0xFF3A3A37), // #3a3a37
    inkP: Color(0xFF000000), // #000000
    dis: Color(0xFFA3A39E), // #a3a39e
    ok: Color(0xFF09672E), // oklch(0.45 0.12 150)
    run: Color(0xFF0053A5), // oklch(0.45 0.15 255)
    fail: Color(0xFFAC1B18), // oklch(0.48 0.18 28)
    focus: Color(0xFF0961BB), // oklch(0.50 0.16 255)
    img: Color(0xFF0891C9), // oklch(0.62 0.13 235)
    pdf: Color(0xFFCE5342), // oklch(0.60 0.16 30)
    zip: Color(0xFFD0A43A), // oklch(0.74 0.13 85)
    fut: Color(0xFF8F8F8A), // #8f8f8a
  );

  static const dark = MorsaTokens(
    paper: Color(0xFF121211), // #121211
    ink: Color(0xFFE8E8E4), // #e8e8e4
    ink2: Color(0xFFA2A29C), // #a2a29c
    weak: Color(0xFF3D3D3A), // #3d3d3a
    weak2: Color(0xFF4A4A46), // #4a4a46
    ctrl: Color(0xFF6A6A65), // #6a6a65
    sunk: Color(0xFF1C1C1A), // #1c1c1a
    sel: Color(0xFF262624), // #262624
    inkH: Color(0xFFFFFFFF), // #ffffff
    inkP: Color(0xFFC9C9C4), // #c9c9c4
    dis: Color(0xFF5F5F5A), // #5f5f5a
    ok: Color(0xFF69C27E), // oklch(0.74 0.13 150)
    run: Color(0xFF75AEF5), // oklch(0.74 0.12 255)
    fail: Color(0xFFF47C6E), // oklch(0.72 0.15 28)
    focus: Color(0xFF65A7FA), // oklch(0.72 0.14 255)
    img: Color(0xFF0891C9), // oklch(0.62 0.13 235)
    pdf: Color(0xFFCE5342), // oklch(0.60 0.16 30)
    zip: Color(0xFFD0A43A), // oklch(0.74 0.13 85)
    fut: Color(0xFF8F8F8A), // #8f8f8a
  );

  @override
  MorsaTokens copyWith() => this;

  @override
  MorsaTokens lerp(MorsaTokens? other, double t) {
    if (other == null) return this;
    return MorsaTokens(
      paper: Color.lerp(paper, other.paper, t)!,
      ink: Color.lerp(ink, other.ink, t)!,
      ink2: Color.lerp(ink2, other.ink2, t)!,
      weak: Color.lerp(weak, other.weak, t)!,
      weak2: Color.lerp(weak2, other.weak2, t)!,
      ctrl: Color.lerp(ctrl, other.ctrl, t)!,
      sunk: Color.lerp(sunk, other.sunk, t)!,
      sel: Color.lerp(sel, other.sel, t)!,
      inkH: Color.lerp(inkH, other.inkH, t)!,
      inkP: Color.lerp(inkP, other.inkP, t)!,
      dis: Color.lerp(dis, other.dis, t)!,
      ok: Color.lerp(ok, other.ok, t)!,
      run: Color.lerp(run, other.run, t)!,
      fail: Color.lerp(fail, other.fail, t)!,
      focus: Color.lerp(focus, other.focus, t)!,
      img: Color.lerp(img, other.img, t)!,
      pdf: Color.lerp(pdf, other.pdf, t)!,
      zip: Color.lerp(zip, other.zip, t)!,
      fut: Color.lerp(fut, other.fut, t)!,
    );
  }
}
