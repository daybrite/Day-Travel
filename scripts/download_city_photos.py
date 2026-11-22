#!/usr/bin/env python3
"""Download the exact photos from the checked-in TravelBookings catalog."""
import concurrent.futures
import hashlib
import json
import subprocess
import unicodedata
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
CATALOG = ROOT / 'data/travelbookings/Cities.json'

def slug(name):
    return unicodedata.normalize('NFKD', name).encode('ascii', 'ignore').decode().lower().replace(' ', '_')

def download(city):
    target = ROOT / f'resource/images/photo_{slug(city["name"])}.jpg'
    temporary = target.with_suffix('.download')
    subprocess.run(['curl', '--fail', '--location', '--silent', '--show-error', '--retry', '3',
                    '--max-time', '90', city['imageURL'], '--output', str(temporary)], check=True)
    raw = temporary.read_bytes()
    if not raw.startswith(b'\xff\xd8\xff'):
        raise ValueError(f'Expected JPEG: {city["name"]}')
    dimensions = subprocess.check_output(['sips', '-g', 'pixelWidth', '-g', 'pixelHeight', str(temporary)], text=True)
    if 'pixelWidth: <nil>' in dimensions or 'pixelHeight: <nil>' in dimensions:
        raise ValueError(f'Cannot decode: {city["name"]}')
    temporary.replace(target)
    print(f'{city["name"]}: {len(raw):,} bytes', flush=True)
    return {'id': city['id'], 'city': city['name'], 'source': city['imageURL'],
            'file': str(target.relative_to(ROOT)), 'bytes': len(raw),
            'sha256': hashlib.sha256(raw).hexdigest()}

if __name__ == '__main__':
    cities = json.loads(CATALOG.read_text())
    with concurrent.futures.ThreadPoolExecutor(max_workers=4) as pool:
        records = list(pool.map(download, cities))
    (CATALOG.parent / 'downloads.json').write_text(json.dumps(records, ensure_ascii=False, indent=2) + '\n')
    print(f'Downloaded {len(records)} photos; {sum(r["bytes"] for r in records):,} total bytes.')
