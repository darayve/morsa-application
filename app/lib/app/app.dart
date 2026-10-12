import 'package:flutter/material.dart';
import 'package:flutter_localizations/flutter_localizations.dart';

import '../design/theme.dart';
import '../tools/img_01/img_01_screen.dart';
import 'l10n/app_localizations.dart';

class MorsaApp extends StatelessWidget {
  const MorsaApp({super.key});

  @override
  Widget build(BuildContext context) {
    return MaterialApp(
      // A barra de título é nativa de cada sistema; o app só define o texto (A4).
      title: 'Morsa',
      debugShowCheckedModeBanner: false,
      theme: buildTheme(Brightness.light),
      darkTheme: buildTheme(Brightness.dark),
      themeMode: ThemeMode.system,
      localizationsDelegates: const [
        AppLocalizations.delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
      ],
      // Inglês primeiro: é o idioma usado quando o do sistema não é suportado (RF30).
      supportedLocales: const [Locale('en'), Locale('pt')],
      localeResolutionCallback: (locale, supported) {
        if (locale?.languageCode == 'pt') return const Locale('pt');
        return const Locale('en');
      },
      home: const Img01Screen(),
    );
  }
}
