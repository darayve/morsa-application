import 'dart:typed_data';

import 'output_writer_stub.dart'
    if (dart.library.io) 'output_writer_io.dart'
    if (dart.library.js_interop) 'output_writer_web.dart'
    as impl;

/// Onde a saída foi parar, para mostrar ao usuário.
class SavedOutput {
  const SavedOutput({required this.fileName, this.fullPath});

  final String fileName;

  /// Caminho completo (desktop). Nulo na web, onde o arquivo vai para os downloads.
  final String? fullPath;
}

/// Grava arquivos de saída. A implementação muda por plataforma:
/// desktop grava em `Documentos/Morsa/`; web dispara um download.
abstract class OutputWriter {
  factory OutputWriter() => impl.createWriter();

  /// `true` quando a saída vai para os downloads do navegador.
  bool get isDownload;

  /// Pasta de saída, para exibir (nulo na web).
  Future<String?> outputFolder();

  /// Grava `bytes` como `<base>.<ext>` sem nunca sobrescrever (RN03).
  /// Lança `ItemFailure` com `E-201` se não conseguir gravar.
  Future<SavedOutput> save(Uint8List bytes, String base, String ext);
}
