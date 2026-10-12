import '../core_api/api.dart';
import 'l10n/app_localizations.dart';

/// Falha de um item, com o código estável do Apêndice A.
class ItemFailure implements Exception {
  const ItemFailure(this.code, {this.params = const {}, this.detail = ''});

  /// Converte qualquer erro em falha do item. Erros do núcleo trazem o
  /// código; qualquer outra coisa (inclusive panic na web) vira `E-199` (RNF12).
  factory ItemFailure.from(Object error) {
    if (error is ItemFailure) return error;
    if (error is MorsaError) {
      return ItemFailure(
        error.code,
        params: {for (final p in error.params) p.key: p.value},
        detail: error.detail,
      );
    }
    return ItemFailure('E-199', detail: error.toString());
  }

  final String code;
  final Map<String, String> params;

  /// Texto técnico, só para o relatório; nunca exibido em destaque.
  final String detail;

  @override
  String toString() => '$code $detail';
}

/// Mensagem e sugestão em linguagem simples para cada código (Apêndice A).
(String, String) failureText(AppLocalizations l, ItemFailure f) {
  return switch (f.code) {
    'E-101' => (l.errorE101, l.suggestE101),
    'E-102' => (l.errorE102, l.suggestE102),
    'E-103' => (l.errorE103, l.suggestE103),
    'E-104' => (l.errorE104, l.suggestE104),
    'E-105' => (l.errorE105, l.suggestE105),
    'E-106' => (
      l.errorE106(f.params['width'] ?? '?', f.params['height'] ?? '?'),
      l.suggestE106,
    ),
    'E-201' => (l.errorE201, l.suggestE201),
    'E-204' => (l.errorE204, l.suggestE204),
    _ => (l.errorE199, l.suggestE199),
  };
}
