import 'dart:io';
import 'dart:typed_data';

import 'package:path/path.dart' as p;
import 'package:path_provider/path_provider.dart';

import '../app/failure.dart';
import 'output_name.dart';
import 'output_writer.dart';

OutputWriter createWriter({String? folder}) => _DesktopWriter(folder);

class _DesktopWriter implements OutputWriter {
  _DesktopWriter(this._folder);

  final String? _folder;

  @override
  bool get isDownload => false;

  /// Pasta padrão: `Morsa` dentro de Documentos (RF10).
  @override
  Future<String?> outputFolder() async {
    if (_folder != null) return _folder;
    final docs = await getApplicationDocumentsDirectory();
    return p.join(docs.path, 'Morsa');
  }

  @override
  Future<SavedOutput> save(Uint8List bytes, String base, String ext) async {
    try {
      final dir = Directory((await outputFolder())!);
      await dir.create(recursive: true);

      final name = nextFreeName(
        base,
        ext,
        (candidate) => File(p.join(dir.path, candidate)).existsSync(),
      );
      final target = p.join(dir.path, name);

      // Gravação atômica (RNF12): escreve num temporário na mesma pasta e só
      // renomeia no fim. Nunca fica um arquivo parcial com o nome final.
      final temp = File(p.join(dir.path, '.$name.morsa-tmp'));
      try {
        await temp.writeAsBytes(bytes, flush: true);
        await temp.rename(target);
      } catch (_) {
        if (await temp.exists()) await temp.delete();
        rethrow;
      }
      return SavedOutput(fileName: name, fullPath: target);
    } on FileSystemException catch (e) {
      throw ItemFailure('E-201', detail: e.toString());
    }
  }
}
