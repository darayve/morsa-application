// ignore: unused_import
import 'package:intl/intl.dart' as intl;

import 'app_localizations.dart';

// ignore_for_file: type=lint

/// The translations for English (`en`).
class AppLocalizationsEn extends AppLocalizations {
  AppLocalizationsEn([String locale = 'en']) : super(locale);

  @override
  String get appTitle => 'Morsa';

  @override
  String get breadcrumbHome => 'Home';

  @override
  String get toolImg01Name => 'Convert images';

  @override
  String get sectionFile => 'File';

  @override
  String get sectionParameters => 'Parameters';

  @override
  String get sectionResult => 'Result';

  @override
  String get openFiles => 'Open files…';

  @override
  String get noFileChosen => 'No file chosen.';

  @override
  String get acceptedFormatsM1 =>
      'PNG · more formats coming in the next versions';

  @override
  String get targetLabel => 'Target';

  @override
  String get qualityLabel => 'Quality';

  @override
  String qualityRangeError(int min, int max) {
    return 'Out of range: use $min to $max.';
  }

  @override
  String get outputFolderLabel => 'Output folder';

  @override
  String get outputFolderWeb => 'Browser downloads';

  @override
  String get convert => 'Convert';

  @override
  String get stateQueued => 'Queued';

  @override
  String get stateRunning => 'Running';

  @override
  String get stateDone => 'Done';

  @override
  String get stateFailed => 'Failed';

  @override
  String formatsAndCount(String from, String to, int count) {
    String _temp0 = intl.Intl.pluralLogic(
      count,
      locale: localeName,
      other: '$count files',
      one: '1 file',
    );
    return '$from → $to · $_temp0';
  }

  @override
  String savedAs(String name) {
    return 'Saved as $name';
  }

  @override
  String downloadedAs(String name) {
    return 'Downloaded as $name';
  }

  @override
  String extensionMismatch(String real, String ext) {
    return 'This file is $real, despite the .$ext extension';
  }

  @override
  String get statusOffline => 'OFFLINE';

  @override
  String get statusNoNetwork => 'No network access';

  @override
  String get statusLocal => 'Local processing';

  @override
  String statusConverting(int done, int total) {
    return 'Converting · $done of $total';
  }

  @override
  String statusVersion(String version) {
    return 'Morsa · v$version';
  }

  @override
  String get errorE101 => 'This conversion is not supported.';

  @override
  String get suggestE101 => 'Choose another target format.';

  @override
  String get errorE102 => 'This file\'s format is not supported.';

  @override
  String get suggestE102 => 'Check that the file is of the expected type.';

  @override
  String get errorE103 => 'The file is empty (0 bytes).';

  @override
  String get suggestE103 => 'Ask the sender for the file again.';

  @override
  String get errorE104 => 'An option is outside the allowed range.';

  @override
  String get suggestE104 => 'Adjust the highlighted value.';

  @override
  String get errorE105 =>
      'The file ended earlier than expected; it may be corrupted.';

  @override
  String get suggestE105 => 'Try exporting the file again at the source.';

  @override
  String errorE106(String width, String height) {
    return 'The image is too large to process safely ($width × $height).';
  }

  @override
  String get suggestE106 =>
      'Shrink the image in another program or use the desktop app.';

  @override
  String get errorE199 =>
      'An unexpected error occurred while processing this file.';

  @override
  String get suggestE199 =>
      'Copy the report and file an issue in the project repository.';

  @override
  String get errorE201 => 'Cannot save to the chosen folder.';

  @override
  String get suggestE201 => 'Choose another output folder.';

  @override
  String get errorE204 =>
      'The original file can no longer be read (it was moved or deleted).';

  @override
  String get suggestE204 => 'Add the file again.';
}
