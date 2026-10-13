// Teste de integração no desktop: app real + núcleo Rust real pela ponte.
//
//   cd app
//   flutter test integration_test -d windows   (ou -d macos, -d linux)
//
// Os arquivos são gravados numa pasta temporária, nunca em Documentos.

import 'dart:io';

import 'package:flutter_test/flutter_test.dart';
import 'package:integration_test/integration_test.dart';
import 'package:morsa/app/app.dart';
import 'package:morsa/app/failure.dart';
import 'package:morsa/core_api/api.dart';
import 'package:morsa/core_api/frb_generated.dart';
import 'package:morsa/storage/output_name.dart';
import 'package:morsa/storage/output_writer.dart';
import 'package:path/path.dart' as p;

/// Arquivo do corpus do repositório. A pasta atual do app muda conforme o
/// sistema (no macOS ela não é a do projeto), então a busca sobe a partir
/// dela e do executável até achar `corpus/SOURCES.md`.
File corpus(String relative) {
  for (final start in [Directory.current.path, Platform.resolvedExecutable]) {
    var dir = Directory(start).absolute;
    while (true) {
      if (File(p.join(dir.path, 'corpus', 'SOURCES.md')).existsSync()) {
        return File(p.join(dir.path, 'corpus', relative));
      }
      if (dir.parent.path == dir.path) break;
      dir = dir.parent;
    }
  }
  throw StateError(
    'Pasta corpus/ não encontrada a partir de ${Directory.current.path}',
  );
}

void main() {
  IntegrationTestWidgetsFlutterBinding.ensureInitialized();

  setUpAll(() async => RustLib.init());

  testWidgets('a tela abre com a ficha "Sem acesso à rede"', (tester) async {
    await tester.pumpWidget(const MorsaApp());
    await tester.pumpAndSettle();
    expect(find.textContaining('IMG-01'), findsOneWidget);
    expect(find.textContaining('OFFLINE'), findsOneWidget);
  });

  testWidgets(
    'PNG → JPG grava arquivo novo, com sufixo, sem tocar no original',
    (tester) async {
      final source = corpus('image/png/valido-rgba.png');
      final before = await source.readAsBytes();
      final temp = await Directory.systemTemp.createTemp('morsa-it-');
      addTearDown(() => temp.delete(recursive: true));
      final writer = OutputWriter(folder: temp.path);

      for (final expected in ['valido-rgba.jpg', 'valido-rgba (1).jpg']) {
        final out = await convertImage(
          input: before,
          settings: defaultSettings(target: FileFormat.jpeg),
        );
        final saved = await writer.save(
          out.bytes,
          outputBaseName('valido-rgba.png'),
          out.outputExtension,
        );
        expect(saved.fileName, expected);
        final written = await File(saved.fullPath!).readAsBytes();
        expect(written.sublist(0, 3), [0xFF, 0xD8, 0xFF]);
      }

      // Nenhum temporário de gravação sobrou na pasta (RNF12).
      final leftovers = temp.listSync().map((e) => p.basename(e.path)).toList()
        ..sort();
      expect(leftovers, ['valido-rgba (1).jpg', 'valido-rgba.jpg']);

      // RN02: o original continua idêntico.
      expect(await source.readAsBytes(), before);
    },
  );

  testWidgets('arquivo truncado vira falha E-105 pela ponte', (tester) async {
    final bytes = await corpus('image/png/truncado.png').readAsBytes();
    Object? error;
    try {
      await convertImage(
        input: bytes,
        settings: defaultSettings(target: FileFormat.jpeg),
      );
    } catch (e) {
      error = e;
    }
    expect(ItemFailure.from(error!).code, 'E-105');
  });
}
