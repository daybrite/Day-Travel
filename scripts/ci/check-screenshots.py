#!/usr/bin/env python3
"""Require a complete city × shipped locale × theme capture set for every device."""
import argparse
import hashlib
import json
from pathlib import Path
import struct
import sys
import unicodedata

ROOT = Path(__file__).resolve().parents[2]
THEMES = ("light", "dark")


def expected(project=ROOT):
    cities = json.loads((project / "data/travelbookings/Cities.json").read_text())
    names = [unicodedata.normalize("NFKD", c["name"]).encode("ascii", "ignore")
             .decode().lower().replace(" ", "_") for c in cities]
    locales = sorted(p.name for p in (project / "resource/locales").iterdir() if p.is_dir())
    return names, locales


def variant(theme, locale):
    # Day's crossed matrix omits the default English suffix.
    return theme if locale == "en" else f"{theme}-{locale}"


def validate(root, target, cities, locales):
    directory = root / target
    variants = {variant(theme, locale) for theme in THEMES for locale in locales}
    # Single-device targets write target/variant; named profiles add target/device/variant.
    devices = {p.parent for p in directory.rglob("*") if p.is_dir() and p.name in variants}
    if not devices:
        raise ValueError(f"{target}: no capture variants found under {directory}")
    records, errors = [], []
    for device in sorted(devices):
        for locale in locales:
            for city in cities:
                pair = []
                for theme in THEMES:
                    path = device / variant(theme, locale) / f"{city}.png"
                    try:
                        data = path.read_bytes()
                        if (data[:8] != b"\x89PNG\r\n\x1a\n" or data[12:16] != b"IHDR"
                                or data[-12:] != b"\0\0\0\0IEND\xaeB`\x82"):
                            raise ValueError("invalid or incomplete PNG")
                        width, height = struct.unpack(">II", data[16:24])
                        if min(width, height) < 200:
                            raise ValueError(f"capture is too small: {width}x{height}")
                        digest = hashlib.sha256(data).hexdigest()
                        pair.append(digest)
                        records.append({"file": str(path.relative_to(root)), "city": city,
                                        "locale": locale, "theme": theme, "width": width,
                                        "height": height, "sha256": digest})
                    except (OSError, ValueError, struct.error) as error:
                        errors.append(f"{path}: {error}")
                if len(pair) == 2 and pair[0] == pair[1]:
                    errors.append(f"{device}/{city}/{locale}: light and dark images are identical")
    if errors:
        raise ValueError("\n".join(errors))
    return records


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--root", type=Path, default=ROOT / "build/day/screenshots")
    parser.add_argument("--target", action="append", required=True)
    args = parser.parse_args()
    cities, locales = expected()
    records = []
    try:
        for target in args.target:
            captured = validate(args.root, target, cities, locales)
            records.extend(captured)
            print(f"{target}: verified {len(captured)} city screenshots "
                  f"({len(cities)} cities × {len(locales)} languages × 2 themes per device)")
    except ValueError as error:
        print(error, file=sys.stderr)
        return 1
    output = args.root / ("coverage-" + "-".join(args.target) + ".json")
    output.write_text(json.dumps({"captures": records}, ensure_ascii=False, indent=2) + "\n")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
