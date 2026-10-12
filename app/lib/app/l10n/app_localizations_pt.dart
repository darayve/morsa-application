// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for Portuguese (`pt`).
class AppLocalizationsPt extends AppLocalizations {
  AppLocalizationsPt([String locale = 'pt']) : super(locale);

  @override
  String get appTitle => 'Morsa';

  @override
  String get breadcrumbHome => 'Início';

  @override
  String get toolImg01Name => 'Converter imagens';

  @override
  String get sectionFile => 'Arquivo';

  @override
  String get sectionParameters => 'Parâmetros';

  @override
  String get sectionResult => 'Resultado';

  @override
  String get openFiles => 'Abrir arquivos…';

  @override
  String get noFileChosen => 'Nenhum arquivo escolhido.';

  @override
  String get acceptedFormatsM1 =>
      'PNG · outros formatos chegam nas próximas versões';

  @override
  String get targetLabel => 'Destino';

  @override
  String get qualityLabel => 'Qualidade';

  @override
  String qualityRangeError(int min, int max) {
    return 'Fora do intervalo: use de $min a $max.';
  }

  @override
  String get outputFolderLabel => 'Pasta de saída';

  @override
  String get outputFolderWeb => 'Downloads do navegador';

  @override
  String get convert => 'Converter';

  @override
  String get stateQueued => 'Na fila';

  @override
  String get stateRunning => 'Executando';

  @override
  String get stateDone => 'Concluído';

  @override
  String get stateFailed => 'Falha';

  @override
  String formatsAndCount(String from, String to, int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count arquivos',
      one: '1 arquivo',
    );
    return '$from → $to · $_temp0';
  }

  @override
  String savedAs(String name) {
    return 'Salvo como $name';
  }

  @override
  String downloadedAs(String name) {
    return 'Baixado como $name';
  }

  @override
  String extensionMismatch(String real, String ext) {
    return 'Este arquivo é $real, apesar da extensão .$ext';
  }

  @override
  String get statusOffline => 'OFFLINE';

  @override
  String get statusNoNetwork => 'Sem acesso à rede';

  @override
  String get statusLocal => 'Processamento local';

  @override
  String statusConverting(int done, int total) {
    return 'Convertendo · $done de $total';
  }

  @override
  String statusVersion(String version) {
    return 'Morsa · v$version';
  }

  @override
  String get errorE101 => 'Esta conversão não é suportada.';

  @override
  String get suggestE101 => 'Escolha outro formato de destino.';

  @override
  String get errorE102 => 'O formato deste arquivo não é suportado.';

  @override
  String get suggestE102 => 'Confira se o arquivo é do tipo esperado.';

  @override
  String get errorE103 => 'O arquivo está vazio (0 bytes).';

  @override
  String get suggestE103 => 'Peça o arquivo novamente a quem o enviou.';

  @override
  String get errorE104 => 'Uma opção está fora do intervalo permitido.';

  @override
  String get suggestE104 => 'Ajuste o valor destacado.';

  @override
  String get errorE105 =>
      'O arquivo terminou antes do esperado; pode estar corrompido.';

  @override
  String get suggestE105 => 'Tente reexportar o arquivo na origem.';

  @override
  String errorE106(String width, String height) {
    return 'A imagem é grande demais para ser processada com segurança ($width × $height).';
  }

  @override
  String get suggestE106 =>
      'Reduza a imagem em outro programa ou use o app desktop.';

  @override
  String get errorE199 =>
      'Ocorreu um erro inesperado ao processar este arquivo.';

  @override
  String get suggestE199 =>
      'Copie o relatório e informe no repositório do projeto.';

  @override
  String get errorE201 => 'Não é possível salvar na pasta escolhida.';

  @override
  String get suggestE201 => 'Escolha outra pasta de destino.';

  @override
  String get errorE204 =>
      'O arquivo original não pode mais ser lido (foi movido ou apagado).';

  @override
  String get suggestE204 => 'Adicione o arquivo novamente.';
}
