{{flutter_js}}
{{flutter_build_config}}

// Por padrão, o Flutter web busca fontes de reserva (Noto) em fonts.gstatic.com
// quando falta um caractere. O Morsa nunca acessa a rede (RNF10): a reserva
// aponta para a própria origem, e as fontes empacotadas cobrem os textos do app.
_flutter.loader.load({
  config: {
    fontFallbackBaseUrl: "assets/fallback-fonts/",
  },
});
