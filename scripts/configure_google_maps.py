#!/usr/bin/env python3
"""Write an ignored Android key resource without printing the credential."""
import os
from pathlib import Path
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parent.parent
ANDROID = "{http://schemas.android.com/apk/res/android}"
key = os.environ.get("DAY_GOOGLE_MAPS_API_KEY", "").strip()
source = "DAY_GOOGLE_MAPS_API_KEY"
if not key:
    for candidate in (
        ROOT.parent / "Day-Showcase/platform/android/AndroidManifest.xml",
        ROOT.parent / "day-piece-map/demo/platform/android/AndroidManifest.xml",
    ):
        if not candidate.exists():
            continue
        for node in ET.parse(candidate).iter("meta-data"):
            if node.get(ANDROID + "name") == "com.google.android.geo.API_KEY":
                value = node.get(ANDROID + "value", "").strip()
                if value and not value.startswith(("@", "${")):
                    key, source = value, str(candidate)
                    break
        if key:
            break
if not key:
    raise SystemExit("Set DAY_GOOGLE_MAPS_API_KEY to an Android Maps SDK key and rerun.")
resources = ET.Element("resources")
ET.SubElement(resources, "string", name="day_google_maps_api_key", translatable="false").text = key
output = ROOT / "platform/android/res/values/maps-local.xml"
output.parent.mkdir(parents=True, exist_ok=True)
ET.ElementTree(resources).write(output, encoding="utf-8", xml_declaration=True)
print(f"Configured local Google Maps resource from {source}; key omitted.")
