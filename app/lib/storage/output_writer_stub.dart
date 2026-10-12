import 'output_writer.dart';

/// Só existe para o compilador; cada plataforma usa _io ou _web.
OutputWriter createWriter({String? folder}) =>
    throw UnsupportedError('Plataforma sem suporte');
