"""Exercise Android's setup resource using synthetic credentials in an isolated checkout."""
import os
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[2]


class MapsSetupTests(unittest.TestCase):
    def setUp(self):
        self.temp = tempfile.TemporaryDirectory()
        self.addCleanup(self.temp.cleanup)
        self.root = Path(self.temp.name) / "app"
        (self.root / "scripts").mkdir(parents=True)
        shutil.copy2(ROOT / "scripts/configure_google_maps.py", self.root / "scripts")

    def run_setup(self, key):
        env = dict(os.environ, DAY_GOOGLE_MAPS_API_KEY=key)
        return subprocess.run([sys.executable, str(self.root / "scripts/configure_google_maps.py")],
                              env=env, capture_output=True, text=True, check=False)

    def test_ci_secret_creates_resource_without_logging_value(self):
        key = "synthetic-test-map-key<&>"
        result = self.run_setup(key)
        self.assertEqual(result.returncode, 0, result.stderr)
        self.assertNotIn(key, result.stdout + result.stderr)
        path = self.root / "platform/android/res/values/maps-local.xml"
        resource = ET.parse(path).getroot().find("string")
        self.assertEqual(resource.attrib["name"], "day_google_maps_api_key")
        self.assertEqual(resource.text, key)

    def test_missing_key_fails_before_build(self):
        result = self.run_setup("")
        self.assertNotEqual(result.returncode, 0)
        self.assertIn("Set DAY_GOOGLE_MAPS_API_KEY", result.stderr)
        self.assertFalse((self.root / "platform/android/res/values/maps-local.xml").exists())


if __name__ == "__main__":
    unittest.main()
