// Gera app/lib/design/tokens.dart a partir da tabela de tokens (requisitos §9.4).
//
// Uso (na raiz do repositório):  dart run scripts/gen_tokens.dart
//
// As cores OKLCH são convertidas para sRGB aqui, em tempo de build, porque o
// Flutter só trabalha com sRGB. Rodar de novo produz o mesmo arquivo.

import 'dart:io';
import 'dart:math' as math;

/// Token → (claro, escuro). Valores em hex ou `oklch(L C H)`.
const tokens = <String, (String, String, String)>{
  'paper': ('#fafaf8', '#121211', 'Fundo'),
  'ink': ('#121212', '#e8e8e4', 'Texto e régua forte'),
  'ink2': ('#5d5d58', '#a2a29c', 'Texto secundário'),
  'weak': ('#dcdcd7', '#3d3d3a', 'Régua fraca'),
  'weak2': ('#cfcfca', '#4a4a46', 'Régua fraca 2, pauta, borda de atalho'),
  'ctrl': ('#121212', '#6a6a65', 'Borda de controle'),
  'sunk': ('#f0f0ec', '#1c1c1a', 'Hover'),
  'sel': ('#e6e6e1', '#262624', 'Linha selecionada'),
  'inkH': ('#3a3a37', '#ffffff', 'Botão primário em hover'),
  'inkP': ('#000000', '#c9c9c4', 'Botão primário pressionado'),
  'dis': ('#a3a39e', '#5f5f5a', 'Desabilitado'),
  'ok': ('oklch(0.45 0.12 150)', 'oklch(0.74 0.13 150)', 'Concluído'),
  'run': ('oklch(0.45 0.15 255)', 'oklch(0.74 0.12 255)', 'Executando'),
  'fail': ('oklch(0.48 0.18 28)', 'oklch(0.72 0.15 28)', 'Falha'),
  'focus': ('oklch(0.50 0.16 255)', 'oklch(0.72 0.14 255)', 'Anel de foco'),
  'img': ('oklch(0.62 0.13 235)', 'oklch(0.62 0.13 235)', 'Etiqueta Imagens'),
  'pdf': ('oklch(0.60 0.16 30)', 'oklch(0.60 0.16 30)', 'Etiqueta PDF'),
  'zip': ('oklch(0.74 0.13 85)', 'oklch(0.74 0.13 85)', 'Etiqueta Compactados'),
  'fut': ('#8f8f8a', '#8f8f8a', 'Etiqueta Em breve'),
};

void main() {
  final names = tokens.keys.toList();
  final b = StringBuffer()
    ..writeln(
      '// GERADO por scripts/gen_tokens.dart a partir dos requisitos §9.4.',
    )
    ..writeln(
      '// Não edite à mão: altere o script e rode `dart run scripts/gen_tokens.dart`.',
    )
    ..writeln()
    ..writeln("import 'package:flutter/material.dart';")
    ..writeln()
    ..writeln('/// Tokens de cor do sistema visual, nos temas claro e escuro.')
    ..writeln('@immutable')
    ..writeln('class MorsaTokens extends ThemeExtension<MorsaTokens> {')
    ..writeln('  const MorsaTokens({');
  for (final n in names) {
    b.writeln('    required this.$n,');
  }
  b
    ..writeln('  });')
    ..writeln();
  for (final n in names) {
    b
      ..writeln('  /// ${tokens[n]!.$3}.')
      ..writeln('  final Color $n;');
  }
  for (final (label, index) in [('light', 0), ('dark', 1)]) {
    b
      ..writeln()
      ..writeln('  static const $label = MorsaTokens(');
    for (final n in names) {
      final src = index == 0 ? tokens[n]!.$1 : tokens[n]!.$2;
      b.writeln('    $n: Color(0x${toArgb(src)}), // $src');
    }
    b.writeln('  );');
  }
  b
    ..writeln()
    ..writeln('  @override')
    ..writeln('  MorsaTokens copyWith() => this;')
    ..writeln()
    ..writeln('  @override')
    ..writeln('  MorsaTokens lerp(MorsaTokens? other, double t) {')
    ..writeln('    if (other == null) return this;')
    ..writeln('    return MorsaTokens(');
  for (final n in names) {
    b.writeln('      $n: Color.lerp($n, other.$n, t)!,');
  }
  b
    ..writeln('    );')
    ..writeln('  }')
    ..writeln('}');

  final out = File.fromUri(
    Platform.script.resolve('../app/lib/design/tokens.dart'),
  );
  out.createSync(recursive: true);
  out.writeAsStringSync(b.toString());
  // Formata com o mesmo Dart que rodou o script, para o arquivo gerado bater
  // byte a byte com o versionado (a CI confere isso).
  final fmt = Process.runSync(Platform.resolvedExecutable, [
    'format',
    out.path,
  ]);
  if (fmt.exitCode != 0) {
    stderr.writeln(fmt.stderr);
    exit(1);
  }
  stdout.writeln('escrito: ${out.path}');
}

/// Converte `#rrggbb` ou `oklch(L C H)` para `FFRRGGBB`.
String toArgb(String value) {
  if (value.startsWith('#')) return 'FF${value.substring(1).toUpperCase()}';
  final parts = value
      .substring(6, value.length - 1)
      .split(' ')
      .map(double.parse)
      .toList();
  final rgb = oklchToSrgb(parts[0], parts[1], parts[2]);
  return 'FF${rgb.map((c) => c.toRadixString(16).padLeft(2, '0')).join().toUpperCase()}';
}

/// OKLCH → OKLab → sRGB linear → sRGB com gama (fórmulas de Björn Ottosson).
/// Cores fora do gamut sRGB são cortadas em 0–1 por canal.
List<int> oklchToSrgb(double l, double c, double hDeg) {
  final h = hDeg * math.pi / 180;
  final a = c * math.cos(h);
  final bb = c * math.sin(h);

  final l1 = math.pow(l + 0.3963377774 * a + 0.2158037573 * bb, 3).toDouble();
  final m1 = math.pow(l - 0.1055613458 * a - 0.0638541728 * bb, 3).toDouble();
  final s1 = math.pow(l - 0.0894841775 * a - 1.2914855480 * bb, 3).toDouble();

  final linear = [
    4.0767416621 * l1 - 3.3077115913 * m1 + 0.2309699292 * s1,
    -1.2684380046 * l1 + 2.6097574011 * m1 - 0.3413193965 * s1,
    -0.0041960863 * l1 - 0.7034186147 * m1 + 1.7076147010 * s1,
  ];

  return linear.map((x) {
    final v = x.clamp(0.0, 1.0);
    final g = v <= 0.0031308 ? 12.92 * v : 1.055 * math.pow(v, 1 / 2.4) - 0.055;
    return (g * 255).round().clamp(0, 255);
  }).toList();
}
