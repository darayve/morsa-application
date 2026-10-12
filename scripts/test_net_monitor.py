"""Testes do analisador do monitor de zero rede.

Uso (qualquer sistema):  python scripts/test_net_monitor.py

Garante que o monitor pega uma tentativa de rede real do app e não acusa
as ferramentas de teste nem os sockets locais.
"""

import unittest

from net_monitor import parse

LOG = """\
100 execve("/opt/flutter/bin/cache/dart-sdk/bin/dart", ["dart"], 0x7ffd /* 30 vars */) = 0
100 connect(9, {sa_family=AF_INET, sin_port=htons(41231), sin_addr=inet_addr("127.0.0.1")}, 16) = 0
100 clone3({flags=CLONE_VM, exit_signal=SIGCHLD, stack=NULL, stack_size=0}, 88 <unfinished ...>
100 <... clone3 resumed> => {parent_tid=[200]}, 88) = 200
200 execve("/home/runner/work/app/build/linux/x64/debug/bundle/morsa", ["morsa"], 0x7ffd /* 30 vars */) = 0
200 connect(5, {sa_family=AF_UNIX, sun_path=@"/tmp/.X11-unix/X99"}, 20) = 0
200 clone(child_stack=0x7f, flags=CLONE_VM|CLONE_FS|CLONE_THREAD) = 201
201 clone3({flags=CLONE_VM|CLONE_THREAD, stack=0x7e, stack_size=0x7fff00}, 88) = 202
202 sendmsg(7, {msg_name={sa_family=AF_UNIX, sun_path="/run/user/1001/bus"}, msg_namelen=110}, 0) = 120
300 connect(4, {sa_family=AF_INET6, sin6_port=htons(443), inet_pton(AF_INET6, "2606:4700::1", &sin6_addr)}, 28) = -1 ENETUNREACH
"""


class ParseTest(unittest.TestCase):
    def test_app_e_threads_sao_monitorados(self):
        app, _ = parse(LOG.splitlines(), "/bundle/morsa")
        self.assertEqual(app, {200, 201, 202})

    def test_sockets_locais_e_ferramenta_nao_contam(self):
        _, violations = parse(LOG.splitlines(), "/bundle/morsa")
        self.assertEqual(violations, [])

    def test_tentativa_ipv4_de_uma_thread_do_app_falha(self):
        log = LOG + (
            '202 connect(11, {sa_family=AF_INET, sin_port=htons(443), '
            'sin_addr=inet_addr("93.184.216.34")}, 16) = -1 ENETUNREACH\n'
        )
        _, violations = parse(log.splitlines(), "/bundle/morsa")
        self.assertEqual(len(violations), 1)
        self.assertIn("pid 202", violations[0])

    def test_dns_por_udp_do_app_falha(self):
        log = LOG + (
            '201 sendto(12, "\\x12\\x34", 32, MSG_NOSIGNAL, {sa_family=AF_INET, '
            'sin_port=htons(53), sin_addr=inet_addr("127.0.0.53")}, 16) = 32\n'
        )
        _, violations = parse(log.splitlines(), "/bundle/morsa")
        self.assertEqual(len(violations), 1)

    def test_conexao_retomada_em_outra_linha_conta(self):
        log = LOG + (
            "200 connect(13, {sa_family=AF_INET6, sin6_port=htons(80)} <unfinished ...>\n"
            "200 <... connect resumed>) = -1 ENETUNREACH\n"
        )
        _, violations = parse(log.splitlines(), "/bundle/morsa")
        self.assertEqual(len(violations), 1)

    def test_sem_o_app_nada_e_monitorado(self):
        app, _ = parse(LOG.splitlines(), "/bundle/outro")
        self.assertEqual(app, set())


if __name__ == "__main__":
    unittest.main()
