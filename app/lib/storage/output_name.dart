import 'package:path/path.dart' as p;

/// Nome base de saída: o nome de origem sem a extensão (Apêndice C, regra 1).
///
/// Um nome como `.png` vira `arquivo`, e não `.png`, para a saída não sair
/// como arquivo oculto (`.png.jpg`).
String outputBaseName(String sourceName) {
  final name = p.basename(sourceName);
  final dot = name.lastIndexOf('.');
  final base = (dot >= 0 ? name.substring(0, dot) : name).trim();
  return base.isEmpty ? 'arquivo' : base;
}

/// Primeiro nome livre para `base.ext`, seguindo a RN03:
/// `foto.jpg` → `foto (1).jpg` → `foto (2).jpg`…
///
/// `exists` diz se um nome já está ocupado na pasta de destino.
String nextFreeName(String base, String ext, bool Function(String) exists) {
  var candidate = '$base.$ext';
  var n = 1;
  while (exists(candidate)) {
    candidate = '$base ($n).$ext';
    n++;
  }
  return candidate;
}
