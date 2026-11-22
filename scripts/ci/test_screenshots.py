"""Capture coverage uses synthetic PNGs; no simulator or map network access required."""
import importlib.util
from pathlib import Path
import re
import struct
import tempfile
import tomllib
import unittest
import zlib

SPEC = importlib.util.spec_from_file_location("coverage", Path(__file__).with_name("check-screenshots.py"))
COVERAGE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(COVERAGE)
ROOT = Path(__file__).resolve().parents[2]


def png(color):
    def chunk(kind, data):
        return (struct.pack(">I", len(data)) + kind + data
                + struct.pack(">I", zlib.crc32(kind + data)))
    return (b"\x89PNG\r\n\x1a\n"
            + chunk(b"IHDR", struct.pack(">IIBBBBB", 200, 200, 8, 2, 0, 0, 0))
            + chunk(b"IDAT", zlib.compress((b"\0" + bytes([color]) * 600) * 200))
            + chunk(b"IEND", b""))


class CaptureCoverageTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name)
        for locale in ("en", "fr"):
            for i, theme in enumerate(COVERAGE.THEMES):
                path = self.root / "ios-uikit/phone" / COVERAGE.variant(theme, locale) / "fixture.png"
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(png(i))

    def validate(self):
        return COVERAGE.validate(self.root, "ios-uikit", ["fixture"], ["en", "fr"])

    def test_complete_named_device_matrix(self):
        self.assertEqual(len(self.validate()), 4)

    def test_missing_city_variant_fails(self):
        (self.root / "ios-uikit/phone/dark-fr/fixture.png").unlink()
        with self.assertRaisesRegex(ValueError, "dark-fr/fixture.png"):
            self.validate()

    def test_truncated_capture_fails(self):
        (self.root / "ios-uikit/phone/light/fixture.png").write_bytes(png(0)[:-4])
        with self.assertRaisesRegex(ValueError, "incomplete PNG"):
            self.validate()

    def test_identical_theme_captures_fail(self):
        (self.root / "ios-uikit/phone/dark/fixture.png").write_bytes(png(0))
        with self.assertRaisesRegex(ValueError, "light and dark images are identical"):
            self.validate()

    def test_empty_target_fails(self):
        with self.assertRaisesRegex(ValueError, "no capture variants"):
            COVERAGE.validate(self.root, "android-material", ["fixture"], ["en"])

    def test_every_device_must_have_a_complete_matrix(self):
        path = self.root / "ios-uikit/tablet/light/fixture.png"
        path.parent.mkdir(parents=True)
        path.write_bytes(png(0))
        with self.assertRaisesRegex(ValueError, "tablet/dark/fixture.png"):
            self.validate()

    def test_capture_scripts_cover_each_catalog_city_exactly_once(self):
        cities, locales = COVERAGE.expected()
        self.assertEqual(len(locales), 7)
        self.assertEqual(len(cities), 28)
        for kind in ("desktop", "mobile"):
            text = (ROOT / f"dayscript/capture-{kind}.yaml").read_text()
            self.assertEqual(re.findall(r"^  - screenshot: (\S+)$", text, re.M), cities)
            self.assertEqual(text.count("assert_visible: { id: city_map }"), len(cities))
            self.assertEqual(text.count("assert_visible: { id: ticket_picker }"), len(cities))
            self.assertNotIn("language_picker", text)
            self.assertNotIn("select_language", text)
            if kind == "mobile":
                indices = re.findall(r"select: \{ id: city_list, index: (\d+) \}", text)
                self.assertEqual([int(i) for i in indices], list(range(len(cities))))
            else:
                self.assertEqual(text.splitlines()[2], "  - wait_for: { id: city_sidebar }")

    def test_ci_covers_all_supported_targets_languages_and_themes(self):
        workflow = (ROOT / ".github/workflows/ci.yml").read_text()
        targets = re.search(r"^      targets: (.+)$", workflow, re.M).group(1)
        supported = set(tomllib.loads((ROOT / "Day.toml").read_text())["app"]["targets"])
        # Legacy XAML is deliberately local-only: the shared workflow supports WinUI.
        self.assertEqual({t.strip() for t in targets.split(",")}, supported - {"windows-xaml"})
        self.assertRegex(workflow, r"(?m)^      locales: all$")
        self.assertRegex(workflow, r"(?m)^      themes: light dark$")
        self.assertIn("script-setup: source scripts/ci/setup-captures.sh", workflow)


if __name__ == "__main__":
    unittest.main()
