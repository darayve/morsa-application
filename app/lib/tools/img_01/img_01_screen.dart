import 'package:file_selector/file_selector.dart';
import 'package:flutter/material.dart';
import 'package:flutter/services.dart';
import 'package:path/path.dart' as p;

import '../../app/failure.dart';
import '../../app/l10n/app_localizations.dart';
import '../../app/platform.dart';
import '../../core_api/api.dart';
import '../../design/status_bar.dart';
import '../../design/theme.dart';
import '../../design/widgets.dart';
import '../../storage/output_name.dart';
import '../../storage/output_writer.dart';

/// IMG-01 · Converter imagens, versão mínima do Marco 1: um PNG → JPG.
///
/// A interface nunca processa o arquivo (§12.2): lê os bytes, entrega ao
/// núcleo Rust e grava o resultado num arquivo novo.
class Img01Screen extends StatefulWidget {
  const Img01Screen({super.key});

  @override
  State<Img01Screen> createState() => _Img01ScreenState();
}

enum _ItemState { idle, running, done, failed }

class _Img01ScreenState extends State<Img01Screen> {
  final _writer = OutputWriter();
  final _quality = TextEditingController(text: '${defaultJpegQuality()}');

  String? _outputFolder;
  XFile? _file;
  Uint8List? _bytes;
  FileFormat? _sourceFormat;
  String? _extensionWarning;

  _ItemState _state = _ItemState.idle;
  ItemFailure? _failure;
  SavedOutput? _saved;

  @override
  void initState() {
    super.initState();
    _writer.outputFolder().then(
      (v) => mounted ? setState(() => _outputFolder = v) : null,
    );
  }

  @override
  void dispose() {
    _quality.dispose();
    super.dispose();
  }

  int? get _qualityValue {
    final v = int.tryParse(_quality.text.trim());
    return (v != null && v >= 1 && v <= 100) ? v : null;
  }

  bool get _canConvert =>
      _bytes != null &&
      _sourceFormat != null &&
      _qualityValue != null &&
      _state != _ItemState.running;

  Future<void> _openFile() async {
    final file = await openFile(
      acceptedTypeGroups: const [
        XTypeGroup(
          label: 'PNG',
          extensions: ['png'],
          mimeTypes: ['image/png'],
          uniformTypeIdentifiers: ['public.png'],
        ),
        // O formato real é decidido pelo conteúdo (RN14), então qualquer
        // arquivo pode ser escolhido; o núcleo recusa o que não reconhece.
        XTypeGroup(label: '*'),
      ],
    );
    if (file == null) return;

    setState(() {
      _file = file;
      _bytes = null;
      _sourceFormat = null;
      _extensionWarning = null;
      _state = _ItemState.idle;
      _failure = null;
      _saved = null;
    });

    try {
      final Uint8List bytes;
      try {
        bytes = await file.readAsBytes();
      } catch (e) {
        throw ItemFailure('E-204', detail: '$e');
      }
      final header = bytes.length > detectHeaderLen()
          ? bytes.sublist(0, detectHeaderLen())
          : bytes;
      final format = await detectFormat(header: header);
      if (!mounted) return;
      setState(() {
        _bytes = bytes;
        _sourceFormat = format;
        _extensionWarning = _mismatch(file.name, format);
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _state = _ItemState.failed;
        _failure = ItemFailure.from(e);
      });
    }
  }

  /// Aviso da RN14 quando a extensão não bate com o conteúdo.
  String? _mismatch(String name, FileFormat real) {
    final ext = p.extension(name).replaceFirst('.', '').toLowerCase();
    final expected = switch (real) {
      FileFormat.png => ['png'],
      FileFormat.jpeg => ['jpg', 'jpeg', 'jpe', 'jfif'],
    };
    if (ext.isEmpty || expected.contains(ext)) return null;
    return AppLocalizations.of(context).extensionMismatch(_label(real), ext);
  }

  Future<void> _convert() async {
    if (!_canConvert) return;
    setState(() {
      _state = _ItemState.running;
      _failure = null;
      _saved = null;
    });
    try {
      final out = await convertImage(
        input: _bytes!,
        target: FileFormat.jpeg,
        quality: _qualityValue!,
      );
      final saved = await _writer.save(
        out.bytes,
        outputBaseName(_file!.name),
        out.outputExtension,
      );
      if (!mounted) return;
      setState(() {
        _state = _ItemState.done;
        _saved = saved;
      });
    } catch (e) {
      if (!mounted) return;
      setState(() {
        _state = _ItemState.failed;
        _failure = ItemFailure.from(e);
      });
    }
  }

  static String _label(FileFormat f) => switch (f) {
    FileFormat.png => 'PNG',
    FileFormat.jpeg => 'JPG',
  };

  @override
  Widget build(BuildContext context) {
    final l = AppLocalizations.of(context);
    final text = Theme.of(context).textTheme;

    return CallbackShortcuts(
      bindings: {
        modShortcut(LogicalKeyboardKey.keyO): _openFile,
        modShortcut(LogicalKeyboardKey.enter): _convert,
      },
      child: Focus(
        autofocus: true,
        child: Scaffold(
          body: Column(
            children: [
              Expanded(
                child: LayoutBuilder(
                  builder: (context, box) {
                    // Janelas estreitas (web): o painel vai para baixo do conteúdo (§9.8).
                    final narrow = box.maxWidth < 720;
                    final content = _content(context, l);
                    final panel = _panel(context, l, narrow);
                    if (narrow) {
                      return SingleChildScrollView(
                        child: Column(children: [content, panel]),
                      );
                    }
                    return Row(
                      crossAxisAlignment: CrossAxisAlignment.stretch,
                      children: [
                        Expanded(child: SingleChildScrollView(child: content)),
                        SizedBox(width: 280, child: panel),
                      ],
                    );
                  },
                ),
              ),
              _actionBar(context, l, text),
              MorsaStatusBar(
                progress: _state == _ItemState.running
                    ? l.statusConverting(0, 1)
                    : null,
              ),
            ],
          ),
        ),
      ),
    );
  }

  Widget _content(BuildContext context, AppLocalizations l) {
    final t = context.tokens;
    final text = Theme.of(context).textTheme;
    return Padding(
      padding: const EdgeInsets.fromLTRB(32, 24, 32, 24),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          Text(
            '${l.breadcrumbHome} · ${l.toolImg01Name}',
            style: text.labelSmall,
          ),
          const SizedBox(height: 4),
          Semantics(
            header: true,
            child: Text.rich(
              TextSpan(
                children: [
                  TextSpan(
                    text: 'IMG-01  ',
                    style: text.headlineMedium!.copyWith(color: t.ink2),
                  ),
                  TextSpan(text: l.toolImg01Name),
                ],
              ),
              style: text.headlineMedium,
            ),
          ),
          const SizedBox(height: 24),
          SectionLabel(l.sectionFile),
          const SizedBox(height: 12),
          Wrap(
            spacing: 12,
            runSpacing: 8,
            crossAxisAlignment: WrapCrossAlignment.center,
            children: [
              MorsaButton(
                label: l.openFiles,
                shortcut: '$modKeyLabel O',
                onPressed: _openFile,
                autofocus: true,
              ),
              Text(
                _file?.name ?? l.noFileChosen,
                style: text.bodyMedium!.copyWith(
                  color: _file == null ? t.ink2 : t.ink,
                ),
              ),
            ],
          ),
          const SizedBox(height: 6),
          Text(l.acceptedFormatsM1, style: text.labelSmall),
          if (_extensionWarning != null) ...[
            const SizedBox(height: 8),
            Text('! $_extensionWarning', style: text.bodySmall),
          ],
          const SizedBox(height: 32),
          SectionLabel(l.sectionResult),
          _resultRow(context, l),
        ],
      ),
    );
  }

  Widget _resultRow(BuildContext context, AppLocalizations l) {
    final t = context.tokens;
    final text = Theme.of(context).textTheme;
    if (_state == _ItemState.idle && _failure == null) {
      return const SizedBox(height: 28);
    }

    final (stateText, stateColor) = switch (_state) {
      _ItemState.running => (l.stateRunning, t.run),
      _ItemState.done => (l.stateDone, t.ok),
      _ItemState.failed => (l.stateFailed, t.fail),
      _ItemState.idle => (l.stateQueued, t.ink2),
    };

    final details = <Widget>[];
    if (_saved != null) {
      details.add(
        Text(
          _writer.isDownload
              ? l.downloadedAs(_saved!.fileName)
              : l.savedAs(_saved!.fileName),
          style: text.bodyMedium,
        ),
      );
    }
    if (_failure != null) {
      final (message, suggestion) = failureText(l, _failure!);
      details
        ..add(Text(message, style: text.bodyMedium!.copyWith(color: t.fail)))
        ..add(Text(suggestion, style: text.bodySmall!.copyWith(color: t.ink2)));
    }

    return Semantics(
      liveRegion: true,
      child: Container(
        padding: const EdgeInsets.symmetric(vertical: 6),
        decoration: BoxDecoration(
          border: Border(bottom: BorderSide(color: t.weak)),
        ),
        child: Row(
          crossAxisAlignment: CrossAxisAlignment.start,
          children: [
            SizedBox(
              width: 40,
              child: Text(
                '001',
                style: text.bodySmall!.copyWith(color: t.ink2),
              ),
            ),
            Expanded(
              child: Column(
                crossAxisAlignment: CrossAxisAlignment.start,
                children: [
                  if (_sourceFormat != null)
                    Text(
                      l.formatsAndCount(_label(_sourceFormat!), 'JPG', 1),
                      style: text.bodyMedium,
                    ),
                  ...details,
                ],
              ),
            ),
            const SizedBox(width: 12),
            // Estado sempre com texto, nunca só cor (RF09).
            Text(
              stateText,
              style: text.labelLarge!.copyWith(color: stateColor),
            ),
          ],
        ),
      ),
    );
  }

  Widget _panel(BuildContext context, AppLocalizations l, bool narrow) {
    final t = context.tokens;
    final qualityError = _qualityValue == null
        ? l.qualityRangeError(1, 100)
        : null;
    return Container(
      decoration: BoxDecoration(
        border: narrow
            ? Border(top: BorderSide(color: t.weak))
            : Border(left: BorderSide(color: t.weak)),
      ),
      padding: const EdgeInsets.fromLTRB(16, 24, 16, 16),
      child: Column(
        crossAxisAlignment: CrossAxisAlignment.start,
        children: [
          SectionLabel(l.sectionParameters),
          FichaRow(label: l.targetLabel, value: const Text('JPG')),
          FichaRow(
            label: l.qualityLabel,
            value: MorsaTextField(
              controller: _quality,
              label: l.qualityLabel,
              errorText: qualityError,
              width: 72,
              keyboardType: TextInputType.number,
              inputFormatters: [
                FilteringTextInputFormatter.digitsOnly,
                LengthLimitingTextInputFormatter(3),
              ],
              onChanged: (_) => setState(() {}),
            ),
          ),
          FichaRow(
            label: l.outputFolderLabel,
            value: Text(
              _writer.isDownload ? l.outputFolderWeb : (_outputFolder ?? '…'),
            ),
          ),
        ],
      ),
    );
  }

  Widget _actionBar(BuildContext context, AppLocalizations l, TextTheme text) {
    final tokens = context.tokens;
    return Container(
      padding: const EdgeInsets.symmetric(horizontal: 28, vertical: 8),
      decoration: BoxDecoration(
        border: Border(top: BorderSide(color: tokens.ink)),
      ),
      child: Row(
        children: [
          Expanded(
            child: Text(
              _sourceFormat == null
                  ? ''
                  : l.formatsAndCount(_label(_sourceFormat!), 'JPG', 1),
              style: text.bodySmall!.copyWith(color: tokens.ink2),
            ),
          ),
          MorsaButton(
            label: l.convert,
            primary: true,
            shortcut: '$modKeyLabel ↵',
            onPressed: _canConvert ? _convert : null,
          ),
        ],
      ),
    );
  }
}
