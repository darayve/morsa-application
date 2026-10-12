import 'package:flutter_test/flutter_test.dart';
import 'package:morsa/storage/output_name.dart';

void main() {
  group('outputBaseName', () {
    test('remove só a última extensão', () {
      expect(outputBaseName('foto.png'), 'foto');
      expect(outputBaseName('relatorio.final.png'), 'relatorio.final');
    });

    test('nome vazio vira "arquivo" (Apêndice C, regra 6)', () {
      expect(outputBaseName('.png'), 'arquivo');
    });
  });

  group('nextFreeName (RN03)', () {
    test('usa o nome direto quando está livre', () {
      expect(nextFreeName('foto', 'jpg', (_) => false), 'foto.jpg');
    });

    test('acrescenta (1), (2)… sem nunca sobrescrever', () {
      final taken = {'foto.jpg', 'foto (1).jpg'};
      expect(nextFreeName('foto', 'jpg', taken.contains), 'foto (2).jpg');
    });
  });
}
