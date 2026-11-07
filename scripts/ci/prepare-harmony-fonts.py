#!/usr/bin/env python3
"""Install CJK fallback in dayapp's disposable CI emulator, then wait for its reboot."""
import os
from pathlib import Path
import subprocess
import time

FONT = Path('/usr/share/fonts/opentype/noto/NotoSansCJK-Regular.ttc')
GUEST_FONT = '/system/fonts/NotoSansCJK-Regular.ttc'


def run(*args, check=True):
    return subprocess.run(['hdc', *args], check=check, capture_output=True, text=True, timeout=15).stdout.strip()


def prepare():
    # This script must never modify an arbitrary connected phone or a developer's
    # long-lived emulator. dayapp unpacks its guest under RUNNER_TEMP before boot.
    if os.environ.get('GITHUB_ACTIONS') != 'true':
        raise RuntimeError('This setup is only for a disposable GitHub Actions emulator')
    temp = Path(os.environ['RUNNER_TEMP']).resolve()
    images = Path(os.environ['DAY_OHOS_EMULATOR']).resolve()
    if images == temp or not images.is_relative_to(temp) or not (images / 'system.img').is_file():
        raise RuntimeError('The emulator images must be inside RUNNER_TEMP')
    target = os.environ['DAY_OHOS_TARGET']
    if not target.startswith('127.0.0.1:') or not target.removeprefix('127.0.0.1:').isdigit():
        raise RuntimeError('Expected the CI emulator on a loopback HDC port')
    if not FONT.is_file():
        raise RuntimeError('Install fonts-noto-cjk on the host before preparing the emulator')
    exists = run('-t', target, 'shell', f'if [ -s {GUEST_FONT} ]; then echo day-font-present; fi')
    if 'day-font-present' in exists:
        print('Noto Sans CJK is already installed in the emulator')
        return
    run('-t', target, 'target', 'mount')
    run('-t', target, 'file', 'send', str(FONT), GUEST_FONT)
    run('-t', target, 'shell', 'chmod', '644', GUEST_FONT)
    # Verify the transfer: hdc versions can report a guest command error with exit 0.
    exists = run('-t', target, 'shell', f'if [ -s {GUEST_FONT} ]; then echo day-font-present; fi')
    if 'day-font-present' not in exists:
        raise RuntimeError('The emulator font transfer did not create a nonempty font file')
    run('-t', target, 'shell', 'reboot', check=False)
    time.sleep(5) # Let the old connection go away before accepting the boot-complete flag.
    deadline = time.monotonic() + 180
    while time.monotonic() < deadline:
        try:
            run('tconn', target, check=False)
            if run('-t', target, 'shell', 'param', 'get', 'bootevent.boot.completed') == 'true':
                print('Emulator rebooted with Noto Sans CJK fallback installed')
                return
        except subprocess.SubprocessError:
            pass
        time.sleep(3)
    raise RuntimeError('The emulator did not finish rebooting after installing its CJK font')


if __name__ == '__main__':
    prepare()
