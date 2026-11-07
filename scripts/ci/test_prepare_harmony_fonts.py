"""Exercise device guards and installation failures without contacting an emulator."""
import importlib.util
import os
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch

spec = importlib.util.spec_from_file_location('fonts', Path(__file__).with_name('prepare-harmony-fonts.py'))
fonts = importlib.util.module_from_spec(spec)
spec.loader.exec_module(fonts)


class FontSetupTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        root = Path(self.temp.name)
        images = root / 'emulator'
        images.mkdir()
        (images / 'system.img').touch()
        font = root / 'font.ttc'
        font.write_bytes(b'synthetic-font-fixture')
        env = {'GITHUB_ACTIONS': 'true', 'RUNNER_TEMP': str(root),
               'DAY_OHOS_EMULATOR': str(images), 'DAY_OHOS_TARGET': '127.0.0.1:55555'}
        for context in (patch.dict(os.environ, env, clear=True), patch.object(fonts, 'FONT', font)):
            context.start()
            self.addCleanup(context.stop)
        self.calls = []
        self.present = False

    def device(self, *args, **kwargs):
        self.calls.append(args)
        if args[-1].startswith('if [ -s '):
            return 'day-font-present' if self.present else ''
        if 'send' in args:
            self.present = True
        if args[-1] == 'bootevent.boot.completed':
            return 'true'
        return ''

    def test_refuses_non_ci_device(self):
        os.environ['GITHUB_ACTIONS'] = 'false'
        with patch.object(fonts, 'run') as run, self.assertRaises(RuntimeError):
            fonts.prepare()
        run.assert_not_called()

    def test_refuses_external_image_directory(self):
        os.environ['DAY_OHOS_EMULATOR'] = '/outside-ci'
        with patch.object(fonts, 'run') as run, self.assertRaises(RuntimeError):
            fonts.prepare()
        run.assert_not_called()

    def test_refuses_remote_device(self):
        os.environ['DAY_OHOS_TARGET'] = '192.0.2.1:55555'
        with patch.object(fonts, 'run') as run, self.assertRaises(RuntimeError):
            fonts.prepare()
        run.assert_not_called()

    def test_existing_font_needs_no_reboot(self):
        self.present = True
        with patch.object(fonts, 'run', side_effect=self.device):
            fonts.prepare()
        self.assertEqual(len(self.calls), 1)

    def test_install_reboots_and_waits(self):
        with patch.object(fonts, 'run', side_effect=self.device), patch.object(fonts.time, 'sleep'):
            fonts.prepare()
        actions = [call[2:] for call in self.calls if call[0] == '-t']
        self.assertIn(('target', 'mount'), actions)
        self.assertIn(('shell', 'chmod', '644', fonts.GUEST_FONT), actions)
        self.assertIn(('shell', 'reboot'), actions)
        self.assertEqual(actions[-1], ('shell', 'param', 'get', 'bootevent.boot.completed'))

    def test_failed_transfer_never_reboots(self):
        with patch.object(fonts, 'run', return_value='') as run, self.assertRaisesRegex(RuntimeError, 'transfer'):
            fonts.prepare()
        self.assertFalse(any('reboot' in call.args for call in run.call_args_list))

    def test_boot_timeout_fails(self):
        def device(*args, **kwargs):
            result = self.device(*args, **kwargs)
            return 'false' if args[-1] == 'bootevent.boot.completed' else result
        with patch.object(fonts, 'run', side_effect=device), patch.object(fonts.time, 'sleep'), \
                patch.object(fonts.time, 'monotonic', side_effect=[0, 1, 181]), \
                self.assertRaisesRegex(RuntimeError, 'rebooting'):
            fonts.prepare()


if __name__ == '__main__':
    unittest.main()
