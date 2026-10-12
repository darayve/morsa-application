// Verifica as dependências Dart do app (requisitos §13.1 e RNF10).
//
// Uso (na raiz do repositório):  dart run scripts/check_dart_deps.dart
//
// 1. Licenças: todo pacote hospedado (pub.dev) usado pelo app, inclusive os de
//    desenvolvimento, precisa ter licença permissiva. GPL, AGPL e LGPL barram.
//    Licença não reconhecida também barra, até alguém revisar e registrar
//    em `revisados` abaixo.
// 2. Rede: nenhum código que vai para o app (dependências de produção e
//    app/lib) pode importar um cliente de rede. Um pacote de rede pode
//    aparecer no grafo sem ser importado (o `file_selector_platform_interface`
//    declara `http` e não usa); nesse caso o compilador o descarta e ele não
//    entra no binário, então a checagem olha os imports, não só a lista.
//
// Sai com código 1 se encontrar qualquer problema.

import 'dart:convert';
import 'dart:io';

/// Pacotes Dart que fazem rede. Nenhum deles pode ser importado pelo app.
const pacotesDeRede = {
  'http',
  'http2',
  'dio',
  'chopper',
  'retrofit',
  'web_socket_channel',
  'web_socket',
  'socket_io_client',
  'grpc',
  'cronet_http',
  'cupertino_http',
  'ok_http',
  'firebase_core',
  'sentry',
  'sentry_flutter',
  'connectivity_plus',
  'url_launcher', // abrir links é do navegador do sistema; só com revisão
};

/// Trechos de código que indicam rede em `app/lib` (dart:io e web).
const usosDeRede = [
  'HttpClient(',
  'Socket.connect',
  'RawSocket.connect',
  'SecureSocket.connect',
  'WebSocket.connect',
  'RawDatagramSocket.bind',
  'XMLHttpRequest',
  'window.fetch',
  '.fetch(',
  'EventSource(',
  'sendBeacon',
];

/// Licença reconhecida pelo texto. A ordem importa: as proibidas vêm antes,
/// para que um arquivo com várias licenças nunca passe por engano.
const proibidas = {
  'AGPL': 'GNU AFFERO GENERAL PUBLIC LICENSE',
  'LGPL': 'GNU LESSER GENERAL PUBLIC LICENSE',
  'LGPL (library)': 'GNU LIBRARY GENERAL PUBLIC LICENSE',
  'GPL': 'GNU GENERAL PUBLIC LICENSE',
};

const permitidas = <String, List<String>>{
  'Apache-2.0': ['Apache License', 'Version 2.0'],
  'BSD-3-Clause': [
    'Redistribution and use in source and binary forms',
    'Neither the name',
  ],
  'BSD-2-Clause': ['Redistribution and use in source and binary forms'],
  'MIT': ['Permission is hereby granted, free of charge'],
  'Zlib': ['This software is provided \'as-is\'', 'altered source versions'],
  'ISC': ['Permission to use, copy, modify, and/or distribute this software'],
};

/// Pacotes cuja licença foi lida por uma pessoa e não é detectável pelo texto.
/// Formato: 'pacote': 'licença · motivo'.
const revisados = <String, String>{};

void main() {
  final appDir = Directory('app');
  if (!File('app/pubspec.yaml').existsSync()) {
    stderr.writeln('Rode na raiz do repositório.');
    exit(2);
  }

  final deps = _pubDeps(appDir);
  final pacotes = {
    for (final p in deps['packages'] as List) (p['name'] as String): p as Map,
  };
  final raiz = pacotes.values.firstWhere((p) => p['kind'] == 'root');
  final producao = _fecho(
    (raiz['directDependencies'] as List).cast<String>(),
    pacotes,
  );
  final caminhos = _caminhos(appDir);

  final problemas = <String>[];

  // 1. Licenças.
  final licencas = <String, String>{};
  for (final p in pacotes.values) {
    final nome = p['name'] as String;
    final fonte = p['source'] as String;
    if (fonte == 'root' || fonte == 'sdk') continue; // SDK do Flutter: BSD-3
    if (fonte == 'path' && _dentroDoRepo(caminhos[nome])) {
      licencas[nome] = 'do projeto (MIT OR Apache-2.0)';
      continue;
    }
    final revisado = revisados[nome];
    if (revisado != null) {
      licencas[nome] = '$revisado (revisado)';
      continue;
    }
    final texto = _textoDaLicenca(caminhos[nome]);
    if (texto == null) {
      problemas.add('$nome: sem arquivo LICENSE');
      continue;
    }
    final proibida = proibidas.entries
        .where((e) => texto.contains(e.value))
        .map((e) => e.key)
        .firstOrNull;
    if (proibida != null) {
      problemas.add('$nome: licença proibida ($proibida)');
      continue;
    }
    final achada = permitidas.entries
        .where((e) => e.value.every(texto.contains))
        .map((e) => e.key)
        .firstOrNull;
    if (achada == null) {
      problemas.add('$nome: licença não reconhecida; revise e registre');
      continue;
    }
    licencas[nome] = achada;
  }

  // 2. Rede: imports de pacotes de rede em código que vai para o app.
  final importsDeRede = RegExp(
    '''(import|export)\\s+['"]package:(${pacotesDeRede.join('|')})/''',
  );
  final fontes = <String, Directory>{
    'app': Directory('app/lib'),
    // Os próprios pacotes de rede ficam de fora: o que importa é quem os usa.
    for (final nome in producao.difference(pacotesDeRede))
      if (caminhos[nome] != null) nome: Directory('${caminhos[nome]}/lib'),
  };
  for (final MapEntry(key: nome, value: lib) in fontes.entries) {
    if (!lib.existsSync()) continue;
    for (final f in lib.listSync(recursive: true).whereType<File>()) {
      if (!f.path.endsWith('.dart')) continue;
      final codigo = f.readAsStringSync();
      final m = importsDeRede.firstMatch(codigo);
      if (m != null) {
        problemas.add('$nome importa package:${m.group(2)} (${f.path})');
      }
    }
  }
  final presentes = producao.intersection(pacotesDeRede);

  // 3. Rede: usos diretos em app/lib (dart:io, APIs do navegador).
  for (final f in Directory('app/lib').listSync(recursive: true)) {
    if (f is! File || !f.path.endsWith('.dart')) continue;
    if (f.path.replaceAll('\\', '/').contains('/core_api/')) continue;
    final codigo = f.readAsStringSync();
    for (final uso in usosDeRede) {
      if (codigo.contains(uso)) problemas.add('app/lib usa $uso (${f.path})');
    }
  }

  // Relatório.
  final ordenados = licencas.keys.toList()..sort();
  stdout.writeln('Licenças (${ordenados.length} pacotes):');
  for (final nome in ordenados) {
    final dev = producao.contains(nome) ? '' : '  [só desenvolvimento]';
    stdout.writeln('  ${nome.padRight(36)} ${licencas[nome]}$dev');
  }
  stdout.writeln();
  stdout.writeln(
    'Rede: ${fontes.length} pacotes de produção verificados '
    '(imports de ${pacotesDeRede.length} clientes de rede conhecidos).',
  );
  if (presentes.isNotEmpty) {
    stdout.writeln(
      '  No grafo, mas sem nenhum import (descartados na compilação): '
      '${presentes.join(', ')}',
    );
  }

  if (problemas.isNotEmpty) {
    stderr.writeln();
    stderr.writeln('FALHOU (${problemas.length}):');
    for (final p in problemas) {
      stderr.writeln('  ✕ $p');
    }
    exit(1);
  }
  stdout.writeln('\nOK: dependências Dart dentro da política.');
}

Map<String, dynamic> _pubDeps(Directory appDir) {
  final r = Process.runSync(
    'flutter',
    ['pub', 'deps', '--json'],
    workingDirectory: appDir.path,
    runInShell: true,
    stdoutEncoding: utf8,
  );
  if (r.exitCode != 0) {
    stderr.writeln(r.stderr);
    exit(2);
  }
  final saida = r.stdout as String;
  return jsonDecode(saida.substring(saida.indexOf('{')))
      as Map<String, dynamic>;
}

/// Todos os pacotes alcançáveis a partir de [inicio].
Set<String> _fecho(List<String> inicio, Map<String, Map> pacotes) {
  final vistos = <String>{};
  final fila = [...inicio];
  while (fila.isNotEmpty) {
    final nome = fila.removeLast();
    if (!vistos.add(nome)) continue;
    fila.addAll(((pacotes[nome]?['dependencies'] as List?) ?? []).cast());
  }
  return vistos;
}

/// Pasta de cada pacote, lida do package_config gerado pelo `pub get`.
Map<String, String> _caminhos(Directory appDir) {
  final config = File('${appDir.path}/.dart_tool/package_config.json');
  final json = jsonDecode(config.readAsStringSync()) as Map;
  final base = config.parent.uri;
  return {
    for (final p in json['packages'] as List)
      p['name'] as String: base
          .resolve(p['rootUri'] as String)
          .toFilePath()
          .replaceAll(RegExp(r'[\\/]$'), ''),
  };
}

bool _dentroDoRepo(String? caminho) {
  if (caminho == null) return false;
  final repo = Directory.current.absolute.path.toLowerCase();
  return File(caminho).absolute.path.toLowerCase().startsWith(repo);
}

String? _textoDaLicenca(String? pasta) {
  if (pasta == null) return null;
  for (final nome in ['LICENSE', 'LICENSE.md', 'LICENSE.txt', 'COPYING']) {
    final f = File('$pasta/$nome');
    if (f.existsSync()) {
      // Junta quebras de linha para as frases baterem mesmo quebradas.
      return f.readAsStringSync().replaceAll(RegExp(r'\s+'), ' ');
    }
  }
  return null;
}
