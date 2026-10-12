// Galeria dos componentes base, só para desenvolvimento.
//
//   cd app
//   flutter run -d windows -t gallery/main.dart   (ou -d macos, -d linux, -d chrome)
//
// Ctrl+T (⌘T no macOS) alterna entre o tema claro e o escuro.

import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:morsa/app/l10n/app_localizations.dart';
import 'package:morsa/app/platform.dart';
import 'package:morsa/design/theme.dart';

import 'gallery.dart';

void main() => runApp(const GalleryApp());

class GalleryApp extends StatefulWidget {
  const GalleryApp({super.key});

  @override
  State<GalleryApp> createState() => _GalleryAppState();
}

class _GalleryAppState extends State<GalleryApp> {
  ThemeMode _mode = ThemeMode.light;

  void _toggle() => setState(
    () => _mode = _mode == ThemeMode.light ? ThemeMode.dark : ThemeMode.light,
  );

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      title: 'Morsa · Componentes',
      debugShowCheckedModeBanner: false,
      theme: buildTheme(Brightness.light),
      darkTheme: buildTheme(Brightness.dark),
      themeMode: _mode,
      locale: const Locale('pt'),
      localizationsDelegates: const [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      supportedLocales: const [Locale('en'), Locale('pt')],
      home: CallbackShortcuts(
        bindings: {modShortcut(LogicalKeyboardKey.keyT): _toggle},
        child: Focus(
          autofocus: true,
          child: ComponentGallery(onToggleTheme: _toggle),
        ),
      ),
    );
  }
}
