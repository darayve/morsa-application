import 'package:flutter/foundation.dart';
import 'package:flutter/services.dart';
import 'package:flutter/widgets.dart';

/// `true` no macOS (inclusive no navegador do macOS): atalhos usam Cmd (A5).
bool get isMac => defaultTargetPlatform == TargetPlatform.macOS;

/// Rótulo da tecla modificadora exibido nos controles: `⌘` ou `Ctrl`.
String get modKeyLabel => isMac ? '⌘' : 'Ctrl';

/// Atalho `Mod+<tecla>`: Cmd no macOS, Ctrl no Windows e Linux (Apêndice B).
SingleActivator modShortcut(LogicalKeyboardKey key) =>
    SingleActivator(key, meta: isMac, control: !isMac);
