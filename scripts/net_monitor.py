"""Monitor de zero rede para o app desktop no Linux (requisitos TST10 a).

Uso (Linux, com strace instalado):
    python3 scripts/net_monitor.py -- <comando que roda o app>

Exemplo, como na CI (rede do sistema já bloqueada por `unshare --net`):
    python3 scripts/net_monitor.py -- xvfb-run -a flutter test integration_test -d linux

Como funciona: o comando roda sob o strace, que registra toda chamada de
sistema de rede (connect, sendto, sendmsg) de todos os processos filhos.
Depois, o script reconstrói a árvore de processos, acha o executável do app
(`.../bundle/morsa`) e falha se ele, ou qualquer processo que ele criou,
tentou falar com um endereço IPv4 ou IPv6, mesmo que a tentativa tenha
falhado por falta de rede. Sockets locais (AF_UNIX: X11, D-Bus) são normais
em apps gráficos e não contam.

As ferramentas de teste (flutter, dart) ficam de fora: elas conversam com o
app pela porta de depuração, e isso não é o app usando a rede.
"""

import argparse
import os
import re
import subprocess
import sys
import tempfile

# "1234 syscall(" ou "1234 <... syscall resumed>"
LINE = re.compile(r"^(\d+)\s+(?:<\.\.\.\s+)?(\w+)(?:\(|\s+resumed>)(.*)$")
RETURN = re.compile(r"=\s+(\d+)(?:\s+<[^>]*>)?\s*$")
EXEC_PATH = re.compile(r'^"([^"]+)"')
INET = re.compile(r"sa_family=AF_INET6?\b")
PROCESS_CREATION = {"clone", "clone3", "fork", "vfork"}
NETWORK_CALLS = {"connect", "sendto", "sendmsg", "sendmmsg"}


def parse(lines, app_suffix):
    """Devolve (pids do app, lista de violações) a partir do log do strace."""
    children = {}  # pid -> [filhos]
    app_roots = set()
    events = []  # (pid, syscall, resto da linha)

    for raw in lines:
        m = LINE.match(raw.rstrip("\n"))
        if not m:
            continue
        pid, call, rest = int(m.group(1)), m.group(2), m.group(3)
        if call in PROCESS_CREATION:
            r = RETURN.search(rest)
            if r:
                children.setdefault(pid, []).append(int(r.group(1)))
        elif call == "execve":
            p = EXEC_PATH.match(rest)
            # Conta mesmo sem o "= 0" na linha (execve interrompido e retomado
            # em outra linha): na dúvida, monitorar mais processos, não menos.
            if p and p.group(1).endswith(app_suffix):
                app_roots.add(pid)
        elif call in NETWORK_CALLS:
            events.append((pid, call, rest))

    app = set()
    stack = list(app_roots)
    while stack:
        pid = stack.pop()
        if pid in app:
            continue
        app.add(pid)
        stack.extend(children.get(pid, []))

    violations = [
        f"pid {pid} · {call}({rest.strip()[:160]}"
        for pid, call, rest in events
        if pid in app and INET.search(rest)
    ]
    return app, violations


def main():
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--app",
        default="/bundle/morsa",
        help="fim do caminho do executável do app (padrão: /bundle/morsa)",
    )
    parser.add_argument("command", nargs=argparse.REMAINDER)
    args = parser.parse_args()
    command = args.command[1:] if args.command[:1] == ["--"] else args.command
    if not command:
        parser.error("informe o comando depois de --")

    with tempfile.TemporaryDirectory() as tmp:
        log = os.path.join(tmp, "strace.log")
        trace = [
            "strace", "-f", "-qq", "-s", "120", "-o", log,
            "-e", "trace=execve,clone,clone3,fork,vfork,connect,sendto,sendmsg,sendmmsg",
            "-e", "signal=none",
        ]
        code = subprocess.call(trace + command)
        with open(log, encoding="utf-8", errors="replace") as f:
            app, violations = parse(f, args.app)

    print(f"\n[zero rede] processos do app monitorados: {len(app)}")
    if not app:
        print(f"[zero rede] FALHOU: o executável '*{args.app}' não foi visto rodando.")
        return 1
    if violations:
        print(f"[zero rede] FALHOU: {len(violations)} tentativa(s) de rede do app:")
        for v in violations:
            print(f"  ✕ {v}")
        return 1
    print("[zero rede] OK: nenhuma tentativa de conexão IPv4/IPv6 pelo app.")
    return code


if __name__ == "__main__":
    sys.exit(main())
