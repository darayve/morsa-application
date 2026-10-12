import 'package:flutter/widgets.dart';

import 'app/app.dart';
import 'core_api/frb_generated.dart';

Future<void> main() async {
  WidgetsFlutterBinding.ensureInitialized();
  // Carrega o núcleo Rust: biblioteca nativa no desktop, WASM no navegador.
  await RustLib.init();
  runApp(const MorsaApp());
}
