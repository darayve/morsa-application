import 'dart:async';

import 'package:flutter/foundation.dart';
import 'package:flutter/widgets.dart';
import 'package:flutter_localizations/flutter_localizations.dart';
import 'package:intl/intl.dart' as intl;

import 'app_localizations_en.dart';
import 'app_localizations_pt.dart';

// ignore_for_file: type=lint

/// Callers can lookup localized strings with an instance of AppLocalizations
/// returned by `AppLocalizations.of(context)`.
///
/// Applications need to include `AppLocalizations.delegate()` in their app's
/// `localizationDelegates` list, and the locales they support in the app's
/// `supportedLocales` list. For example:
///
/// ```dart
/// import 'l10n/app_localizations.dart';
///
/// return MaterialApp(
///   localizationsDelegates: AppLocalizations.localizationsDelegates,
///   supportedLocales: AppLocalizations.supportedLocales,
///   home: MyApplicationHome(),
/// );
/// ```
///
/// ## Update pubspec.yaml
///
/// Please make sure to update your pubspec.yaml to include the following
/// packages:
///
/// ```yaml
/// dependencies:
///   # Internationalization support.
///   flutter_localizations:
///     sdk: flutter
///   intl: any # Use the pinned version from flutter_localizations
///
///   # Rest of dependencies
/// ```
///
/// ## iOS Applications
///
/// iOS applications define key application metadata, including supported
/// locales, in an Info.plist file that is built into the application bundle.
/// To configure the locales supported by your app, you’ll need to edit this
/// file.
///
/// First, open your project’s ios/Runner.xcworkspace Xcode workspace file.
/// Then, in the Project Navigator, open the Info.plist file under the Runner
/// project’s Runner folder.
///
/// Next, select the Information Property List item, select Add Item from the
/// Editor menu, then select Localizations from the pop-up menu.
///
/// Select and expand the newly-created Localizations item then, for each
/// locale your application supports, add a new item and select the locale
/// you wish to add from the pop-up menu in the Value field. This list should
/// be consistent with the languages listed in the AppLocalizations.supportedLocales
/// property.
abstract class AppLocalizations {
  AppLocalizations(String locale)
    : localeName = intl.Intl.canonicalizedLocale(locale.toString());

  final String localeName;

  static AppLocalizations of(BuildContext context) {
    return Localizations.of<AppLocalizations>(context, AppLocalizations)!;
  }

  static const LocalizationsDelegate<AppLocalizations> delegate =
      _AppLocalizationsDelegate();

  /// A list of this localizations delegate along with the default localizations
  /// delegates.
  ///
  /// Returns a list of localizations delegates containing this delegate along with
  /// GlobalMaterialLocalizations.delegate, GlobalCupertinoLocalizations.delegate,
  /// and GlobalWidgetsLocalizations.delegate.
  ///
  /// Additional delegates can be added by appending to this list in
  /// MaterialApp. This list does not have to be used at all if a custom list
  /// of delegates is preferred or required.
  static const List<LocalizationsDelegate<dynamic>> localizationsDelegates =
      <LocalizationsDelegate<dynamic>>[
        delegate,
        GlobalMaterialLocalizations.delegate,
        GlobalCupertinoLocalizations.delegate,
        GlobalWidgetsLocalizations.delegate,
      ];

  /// A list of this localizations delegate's supported locales.
  static const List<Locale> supportedLocales = <Locale>[
    Locale('en'),
    Locale('pt'),
  ];

  /// No description provided for @appTitle.
  ///
  /// In pt, this message translates to:
  /// **'Morsa'**
  String get appTitle;

  /// No description provided for @breadcrumbHome.
  ///
  /// In pt, this message translates to:
  /// **'Início'**
  String get breadcrumbHome;

  /// No description provided for @toolImg01Name.
  ///
  /// In pt, this message translates to:
  /// **'Converter imagens'**
  String get toolImg01Name;

  /// No description provided for @sectionFile.
  ///
  /// In pt, this message translates to:
  /// **'Arquivo'**
  String get sectionFile;

  /// No description provided for @sectionParameters.
  ///
  /// In pt, this message translates to:
  /// **'Parâmetros'**
  String get sectionParameters;

  /// No description provided for @sectionResult.
  ///
  /// In pt, this message translates to:
  /// **'Resultado'**
  String get sectionResult;

  /// No description provided for @openFiles.
  ///
  /// In pt, this message translates to:
  /// **'Abrir arquivos…'**
  String get openFiles;

  /// No description provided for @noFileChosen.
  ///
  /// In pt, this message translates to:
  /// **'Nenhum arquivo escolhido.'**
  String get noFileChosen;

  /// No description provided for @acceptedFormatsM1.
  ///
  /// In pt, this message translates to:
  /// **'PNG · outros formatos chegam nas próximas versões'**
  String get acceptedFormatsM1;

  /// No description provided for @targetLabel.
  ///
  /// In pt, this message translates to:
  /// **'Destino'**
  String get targetLabel;

  /// No description provided for @qualityLabel.
  ///
  /// In pt, this message translates to:
  /// **'Qualidade'**
  String get qualityLabel;

  /// No description provided for @qualityRangeError.
  ///
  /// In pt, this message translates to:
  /// **'Fora do intervalo: use de {min} a {max}.'**
  String qualityRangeError(int min, int max);

  /// No description provided for @outputFolderLabel.
  ///
  /// In pt, this message translates to:
  /// **'Pasta de saída'**
  String get outputFolderLabel;

  /// No description provided for @outputFolderWeb.
  ///
  /// In pt, this message translates to:
  /// **'Downloads do navegador'**
  String get outputFolderWeb;

  /// No description provided for @convert.
  ///
  /// In pt, this message translates to:
  /// **'Converter'**
  String get convert;

  /// No description provided for @stateQueued.
  ///
  /// In pt, this message translates to:
  /// **'Na fila'**
  String get stateQueued;

  /// No description provided for @stateRunning.
  ///
  /// In pt, this message translates to:
  /// **'Executando'**
  String get stateRunning;

  /// No description provided for @stateDone.
  ///
  /// In pt, this message translates to:
  /// **'Concluído'**
  String get stateDone;

  /// No description provided for @stateFailed.
  ///
  /// In pt, this message translates to:
  /// **'Falha'**
  String get stateFailed;

  /// No description provided for @formatsAndCount.
  ///
  /// In pt, this message translates to:
  /// **'{from} → {to} · {count, plural, =1{1 arquivo} other{{count} arquivos}}'**
  String formatsAndCount(String from, String to, int count);

  /// No description provided for @savedAs.
  ///
  /// In pt, this message translates to:
  /// **'Salvo como {name}'**
  String savedAs(String name);

  /// No description provided for @downloadedAs.
  ///
  /// In pt, this message translates to:
  /// **'Baixado como {name}'**
  String downloadedAs(String name);

  /// No description provided for @extensionMismatch.
  ///
  /// In pt, this message translates to:
  /// **'Este arquivo é {real}, apesar da extensão .{ext}'**
  String extensionMismatch(String real, String ext);

  /// No description provided for @statusOffline.
  ///
  /// In pt, this message translates to:
  /// **'OFFLINE'**
  String get statusOffline;

  /// No description provided for @statusNoNetwork.
  ///
  /// In pt, this message translates to:
  /// **'Sem acesso à rede'**
  String get statusNoNetwork;

  /// No description provided for @statusLocal.
  ///
  /// In pt, this message translates to:
  /// **'Processamento local'**
  String get statusLocal;

  /// No description provided for @statusConverting.
  ///
  /// In pt, this message translates to:
  /// **'Convertendo · {done} de {total}'**
  String statusConverting(int done, int total);

  /// No description provided for @statusVersion.
  ///
  /// In pt, this message translates to:
  /// **'Morsa · v{version}'**
  String statusVersion(String version);

  /// No description provided for @errorE101.
  ///
  /// In pt, this message translates to:
  /// **'Esta conversão não é suportada.'**
  String get errorE101;

  /// No description provided for @suggestE101.
  ///
  /// In pt, this message translates to:
  /// **'Escolha outro formato de destino.'**
  String get suggestE101;

  /// No description provided for @errorE102.
  ///
  /// In pt, this message translates to:
  /// **'O formato deste arquivo não é suportado.'**
  String get errorE102;

  /// No description provided for @suggestE102.
  ///
  /// In pt, this message translates to:
  /// **'Confira se o arquivo é do tipo esperado.'**
  String get suggestE102;

  /// No description provided for @errorE103.
  ///
  /// In pt, this message translates to:
  /// **'O arquivo está vazio (0 bytes).'**
  String get errorE103;

  /// No description provided for @suggestE103.
  ///
  /// In pt, this message translates to:
  /// **'Peça o arquivo novamente a quem o enviou.'**
  String get suggestE103;

  /// No description provided for @errorE104.
  ///
  /// In pt, this message translates to:
  /// **'Uma opção está fora do intervalo permitido.'**
  String get errorE104;

  /// No description provided for @suggestE104.
  ///
  /// In pt, this message translates to:
  /// **'Ajuste o valor destacado.'**
  String get suggestE104;

  /// No description provided for @errorE105.
  ///
  /// In pt, this message translates to:
  /// **'O arquivo terminou antes do esperado; pode estar corrompido.'**
  String get errorE105;

  /// No description provided for @suggestE105.
  ///
  /// In pt, this message translates to:
  /// **'Tente reexportar o arquivo na origem.'**
  String get suggestE105;

  /// No description provided for @errorE106.
  ///
  /// In pt, this message translates to:
  /// **'A imagem é grande demais para ser processada com segurança ({width} × {height}).'**
  String errorE106(String width, String height);

  /// No description provided for @suggestE106.
  ///
  /// In pt, this message translates to:
  /// **'Reduza a imagem em outro programa ou use o app desktop.'**
  String get suggestE106;

  /// No description provided for @errorE199.
  ///
  /// In pt, this message translates to:
  /// **'Ocorreu um erro inesperado ao processar este arquivo.'**
  String get errorE199;

  /// No description provided for @suggestE199.
  ///
  /// In pt, this message translates to:
  /// **'Copie o relatório e informe no repositório do projeto.'**
  String get suggestE199;

  /// No description provided for @errorE201.
  ///
  /// In pt, this message translates to:
  /// **'Não é possível salvar na pasta escolhida.'**
  String get errorE201;

  /// No description provided for @suggestE201.
  ///
  /// In pt, this message translates to:
  /// **'Escolha outra pasta de destino.'**
  String get suggestE201;

  /// No description provided for @errorE204.
  ///
  /// In pt, this message translates to:
  /// **'O arquivo original não pode mais ser lido (foi movido ou apagado).'**
  String get errorE204;

  /// No description provided for @suggestE204.
  ///
  /// In pt, this message translates to:
  /// **'Adicione o arquivo novamente.'**
  String get suggestE204;
}

class _AppLocalizationsDelegate
    extends LocalizationsDelegate<AppLocalizations> {
  const _AppLocalizationsDelegate();

  @override
  Future<AppLocalizations> load(Locale locale) {
    return SynchronousFuture<AppLocalizations>(lookupAppLocalizations(locale));
  }

  @override
  bool isSupported(Locale locale) =>
      <String>['en', 'pt'].contains(locale.languageCode);

  @override
  bool shouldReload(_AppLocalizationsDelegate old) => false;
}

AppLocalizations lookupAppLocalizations(Locale locale) {
  // Lookup logic when only language code is specified.
  switch (locale.languageCode) {
    case 'en':
      return AppLocalizationsEn();
    case 'pt':
      return AppLocalizationsPt();
  }

  throw FlutterError(
    'AppLocalizations.delegate failed to load unsupported locale "$locale". This is likely '
    'an issue with the localizations generation tool. Please file an issue '
    'on GitHub with a reproducible sample app and the gen-l10n configuration '
    'that was used.',
  );
}
