import 'dart:js_interop';
import 'dart:typed_data';

import 'package:web/web.dart' as web;

import 'output_writer.dart';

OutputWriter createWriter({String? folder}) => _BrowserWriter();

/// Na web, a saída vira um download gerado localmente (Blob): nada sai do
/// navegador. A gravação direta numa pasta (File System Access API) e o ZIP
/// para lotes ficam para o Marco 3 (RF10).
class _BrowserWriter implements OutputWriter {
  @override
  bool get isDownload => true;

  @override
  Future<String?> outputFolder() async => null;

  @override
  Future<SavedOutput> save(Uint8List bytes, String base, String ext) async {
    final name = '$base.$ext';
    final blob = web.Blob(
      [bytes.toJS].toJS,
      web.BlobPropertyBag(
        type: ext == 'jpg' ? 'image/jpeg' : 'application/octet-stream',
      ),
    );
    final url = web.URL.createObjectURL(blob);
    final anchor = web.HTMLAnchorElement()
      ..href = url
      ..download = name;
    web.document.body!.append(anchor);
    anchor.click();
    anchor.remove();
    web.URL.revokeObjectURL(url);
    return SavedOutput(fileName: name);
  }
}
