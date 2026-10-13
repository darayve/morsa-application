// Componentes base (DOC-01 §5): estados, teclado, semântica e contraste.

import 'dart:math' as math;

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:flutter_test/flutter_test.dart';
import 'package:morsa/app/l10n/app_localizations.dart';
import 'package:morsa/design/controls.dart';
import 'package:morsa/design/feedback.dart';
import 'package:morsa/design/table.dart';
import 'package:morsa/design/theme.dart';
import 'package:morsa/design/tokens.dart';
import 'package:morsa/design/widgets.dart';

import '../../gallery/gallery.dart';

Widget wrap(Widget child, {Brightness brightness = Brightness.light}) {
  return MaterialApp(
    theme: buildTheme(brightness),
    locale: const Locale('pt'),
    localizationsDelegates: const [
      AppLocalizations.delegate,
      GlobalMaterialLocalizations.delegate,
      GlobalWidgetsLocalizations.delegate,
      GlobalCupertinoLocalizations.delegate,
    ],
    supportedLocales: const [Locale('en'), Locale('pt')],
    home: Scaffold(body: Center(child: child)),
  );
}

/// Contraste WCAG entre duas cores opacas.
double contrast(Color a, Color b) {
  double lum(Color c) {
    double ch(double v) =>
        v <= 0.04045 ? v / 12.92 : math.pow((v + 0.055) / 1.055, 2.4) * 1.0;
    return 0.2126 * ch(c.r) + 0.7152 * ch(c.g) + 0.0722 * ch(c.b);
  }

  final (x, y) = (lum(a), lum(b));
  return (math.max(x, y) + 0.05) / (math.min(x, y) + 0.05);
}

void main() {
  group('galeria', () {
    for (final brightness in Brightness.values) {
      testWidgets(
        'todos os componentes desenham sem erro (${brightness.name})',
        (tester) async {
          tester.view.physicalSize = const Size(1440, 3200);
          tester.view.devicePixelRatio = 1;
          addTearDown(tester.view.reset);
          await tester.pumpWidget(
            wrap(const ComponentGallery(), brightness: brightness),
          );
          await tester.pumpAndSettle();
          expect(tester.takeException(), isNull);
          expect(find.text('DOC-01 · Componentes base'), findsOneWidget);
        },
      );
    }
  });

  group('MorsaButton', () {
    testWidgets('clique, Enter e Espaço acionam; desabilitado não', (
      tester,
    ) async {
      var n = 0;
      final focus = FocusNode();
      addTearDown(focus.dispose);
      await tester.pumpWidget(
        wrap(
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              MorsaButton(
                label: 'Converter',
                focusNode: focus,
                onPressed: () => n++,
              ),
              const MorsaButton(label: 'Parado', onPressed: null),
            ],
          ),
        ),
      );
      await tester.tap(find.text('Converter'));
      expect(n, 1);

      focus.requestFocus();
      await tester.pump();
      await tester.sendKeyEvent(LogicalKeyboardKey.enter);
      await tester.sendKeyEvent(LogicalKeyboardKey.space);
      expect(n, 3);

      await tester.tap(find.text('Parado'), warnIfMissed: false);
      expect(n, 3);
    });

    testWidgets('leitor de tela vê um botão e consegue acioná-lo', (
      tester,
    ) async {
      final handle = tester.ensureSemantics();
      var n = 0;
      await tester.pumpWidget(
        wrap(MorsaButton(label: 'Converter', onPressed: () => n++)),
      );
      final node = tester.getSemantics(find.byType(MorsaButton));
      expect(
        node,
        isSemantics(
          label: 'Converter',
          isButton: true,
          isEnabled: true,
          isFocusable: true,
          hasTapAction: true,
        ),
      );
      tester.semantics.tap(find.semantics.byLabel('Converter'));
      expect(n, 1);
      handle.dispose();
    });
  });

  group('MorsaCheckbox', () {
    testWidgets('alterna; o parcial vira marcado', (tester) async {
      bool? value;
      await tester.pumpWidget(
        StatefulBuilder(
          builder: (context, set) => wrap(
            MorsaCheckbox(
              value: value,
              label: 'Manter metadados',
              onChanged: (v) => set(() => value = v),
            ),
          ),
        ),
      );
      await tester.tap(find.text('Manter metadados'));
      await tester.pump();
      expect(value, isTrue);
      await tester.tap(find.text('Manter metadados'));
      await tester.pump();
      expect(value, isFalse);
    });

    testWidgets('semântica: marcado e parcial', (tester) async {
      final handle = tester.ensureSemantics();
      await tester.pumpWidget(
        wrap(
          Column(
            mainAxisSize: MainAxisSize.min,
            children: [
              MorsaCheckbox(value: true, label: 'A', onChanged: (_) {}),
              MorsaCheckbox(value: null, label: 'B', onChanged: (_) {}),
            ],
          ),
        ),
      );
      expect(
        tester.getSemantics(find.byType(MorsaCheckbox).first),
        isSemantics(label: 'A', hasCheckedState: true, isChecked: true),
      );
      expect(
        tester.getSemantics(find.byType(MorsaCheckbox).last),
        isSemantics(label: 'B', hasCheckedState: true, isCheckStateMixed: true),
      );
      handle.dispose();
    });
  });

  testWidgets('MorsaSegmented: setas mudam a escolha e pulam o desabilitado', (
    tester,
  ) async {
    var value = 'PNG';
    await tester.pumpWidget(
      StatefulBuilder(
        builder: (context, set) => wrap(
          MorsaSegmented<String>(
            value: value,
            onChanged: (v) => set(() => value = v),
            options: const [
              MorsaOption('PNG', 'PNG'),
              MorsaOption('TIFF', 'TIFF', enabled: false),
              MorsaOption('JPG', 'JPG'),
            ],
          ),
        ),
      ),
    );
    await tester.tap(find.text('TIFF'), warnIfMissed: false);
    expect(value, 'PNG');

    await tester.sendKeyEvent(LogicalKeyboardKey.tab);
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowRight);
    await tester.pump();
    expect(value, 'JPG');
    await tester.sendKeyEvent(LogicalKeyboardKey.arrowLeft);
    await tester.pump();
    expect(value, 'PNG');
  });

  testWidgets('MorsaSelect: abre o menu e escolhe uma opção', (tester) async {
    var value = 'png';
    await tester.pumpWidget(
      StatefulBuilder(
        builder: (context, set) => wrap(
          MorsaSelect<String>(
            label: 'Destino',
            value: value,
            onChanged: (v) => set(() => value = v),
            options: const [
              MorsaOption('png', 'PNG · sem perdas'),
              MorsaOption('jpg', 'JPG · com perdas'),
            ],
          ),
        ),
      ),
    );
    await tester.tap(find.text('PNG · sem perdas'));
    await tester.pumpAndSettle();
    await tester.tap(find.text('JPG · com perdas').last);
    await tester.pumpAndSettle();
    expect(value, 'jpg');
    expect(find.text('JPG · com perdas'), findsOneWidget);
  });

  group('MorsaTextField', () {
    testWidgets('erro aparece com ✕ embaixo do campo', (tester) async {
      final c = TextEditingController(text: '1200');
      addTearDown(c.dispose);
      await tester.pumpWidget(
        wrap(
          MorsaTextField(
            controller: c,
            label: 'DPI',
            suffix: 'DPI',
            numeric: true,
            errorText: 'Fora do intervalo: use de 72 a 600.',
          ),
        ),
      );
      expect(
        find.text('✕ Fora do intervalo: use de 72 a 600.'),
        findsOneWidget,
      );
      expect(find.text('DPI'), findsOneWidget);
    });

    testWidgets('desabilitado não aceita texto', (tester) async {
      final c = TextEditingController(text: '~/Saída');
      addTearDown(c.dispose);
      await tester.pumpWidget(
        wrap(MorsaTextField(controller: c, label: 'Pasta', enabled: false)),
      );
      await tester.enterText(find.byType(TextField), 'outra');
      expect(c.text, '~/Saída');
    });
  });

  testWidgets('MorsaTableRow: clique e Espaço marcam a linha', (tester) async {
    var selected = false;
    final key = GlobalKey();
    await tester.pumpWidget(
      StatefulBuilder(
        builder: (context, set) => wrap(
          SizedBox(
            width: 600,
            child: MorsaTableRow(
              key: key,
              columns: const [MorsaColumn.fixed(52), MorsaColumn.flex()],
              cells: const [Text('001'), Text('capa.png')],
              selected: selected,
              onSelectedChanged: (v) => set(() => selected = v),
            ),
          ),
        ),
      ),
    );
    await tester.tap(find.text('capa.png'));
    await tester.pump();
    expect(selected, isTrue);

    await tester.sendKeyEvent(LogicalKeyboardKey.tab);
    await tester.sendKeyEvent(LogicalKeyboardKey.space);
    await tester.pump();
    expect(selected, isFalse);
  });

  testWidgets('MorsaProgressBar informa a porcentagem ao leitor de tela', (
    tester,
  ) async {
    final handle = tester.ensureSemantics();
    await tester.pumpWidget(
      wrap(
        const SizedBox(
          width: 200,
          child: MorsaProgressBar(value: 0.46, semanticLabel: 'capa.png'),
        ),
      ),
    );
    expect(
      tester.getSemantics(find.byType(MorsaProgressBar)),
      isSemantics(label: 'capa.png', value: '46%'),
    );
    handle.dispose();
  });

  group('contraste (WCAG 1.4.3, DOC-01 §2)', () {
    for (final (name, t) in [
      ('claro', MorsaTokens.light),
      ('escuro', MorsaTokens.dark),
    ]) {
      test('texto e estados passam 4,5:1 sobre o papel ($name)', () {
        for (final (token, color) in [
          ('ink', t.ink),
          ('ink2', t.ink2),
          ('ok', t.ok),
          ('run', t.run),
          ('fail', t.fail),
        ]) {
          expect(
            contrast(color, t.paper),
            greaterThanOrEqualTo(4.5),
            reason: '$token sobre paper',
          );
        }
      });

      test('anel de foco passa 3:1 sobre o papel ($name)', () {
        expect(contrast(t.focus, t.paper), greaterThanOrEqualTo(3));
      });
    }
  });
}
