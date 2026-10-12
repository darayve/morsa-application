"""Serve a build web do Morsa localmente (requisitos §16.3).

Uso (na raiz do repositório, depois de `flutter build web`):
    python scripts/serve_web.py            # serve app/build/web em http://localhost:8080
    python scripts/serve_web.py 9000       # outra porta

Por que não `python -m http.server`: o núcleo Rust roda em Web Workers com
memória compartilhada, e o navegador só libera isso com os cabeçalhos
Cross-Origin-Opener-Policy e Cross-Origin-Embedder-Policy. Este servidor
acrescenta esses cabeçalhos e serve só arquivos locais, só em localhost.
"""

import functools
import http.server
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent / "app" / "build" / "web"


class Handler(http.server.SimpleHTTPRequestHandler):
    extensions_map = {
        **http.server.SimpleHTTPRequestHandler.extensions_map,
        ".wasm": "application/wasm",
        ".js": "text/javascript",
        ".mjs": "text/javascript",
    }

    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        self.send_header("Cache-Control", "no-store")
        super().end_headers()


def main():
    port = int(sys.argv[1]) if len(sys.argv) > 1 else 8080
    if not (ROOT / "index.html").exists():
        sys.exit(f"Build web não encontrada em {ROOT}. Rode antes: flutter build web --release --no-web-resources-cdn")
    handler = functools.partial(Handler, directory=str(ROOT))
    with http.server.ThreadingHTTPServer(("127.0.0.1", port), handler) as server:
        print(f"Morsa em http://localhost:{port}  (Ctrl+C para parar)")
        server.serve_forever()


if __name__ == "__main__":
    main()
