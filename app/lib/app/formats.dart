import '../core_api/api.dart';

/// Nome curto de cada formato na interface (`PNG → JPG`).
String formatLabel(FileFormat f) => switch (f) {
  FileFormat.png => 'PNG',
  FileFormat.jpeg => 'JPG',
  FileFormat.webp => 'WebP',
  FileFormat.gif => 'GIF',
  FileFormat.bmp => 'BMP',
  FileFormat.tiff => 'TIFF',
  FileFormat.ico => 'ICO',
  FileFormat.svg => 'SVG',
};

/// Extensões aceitas na entrada (§5.1). Servem só para avisar quando a
/// extensão não bate com o conteúdo (RN14); o formato real vem dos bytes.
List<String> inputExtensions(FileFormat f) => switch (f) {
  FileFormat.png => ['png'],
  FileFormat.jpeg => ['jpg', 'jpeg', 'jpe', 'jfif'],
  FileFormat.webp => ['webp'],
  FileFormat.gif => ['gif'],
  FileFormat.bmp => ['bmp', 'dib'],
  FileFormat.tiff => ['tif', 'tiff'],
  FileFormat.ico => ['ico'],
  FileFormat.svg => ['svg'],
};

extension ConvertSettingsCopy on ConvertSettings {
  /// Cópia com outra qualidade (os campos gerados pela ponte são `final`).
  ConvertSettings withQuality(int quality) => ConvertSettings(
    target: target,
    quality: quality,
    webpLossless: webpLossless,
    background: background,
    icoSizes: this.icoSizes,
    tiffCompression: tiffCompression,
    svgSize: svgSize,
  );
}
