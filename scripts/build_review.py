#!/usr/bin/env python3
"""Assemble local PNGs into a portable, offline review gallery. No dependencies."""
import argparse
import hashlib
import html
import json
from pathlib import Path
import shutil
import struct
from datetime import datetime, timezone

APP = Path(__file__).resolve().parents[1]
TARGETS = [
    ('macos-appkit', 'macOS · AppKit', False),
    ('macos-gtk', 'macOS · GTK', False),
    ('macos-qt', 'macOS · Qt', False),
    ('ios-uikit', 'iOS · UIKit', True),
    ('android-material', 'Android · Material', True),
    ('linux-gtk', 'Linux · GTK', False),
    ('linux-qt', 'Linux · Qt', False),
    ('windows-winui', 'Windows · WinUI', False),
    ('windows-xaml', 'Windows · XAML (legacy)', False),
    ('harmony-arkui', 'HarmonyOS · ArkUI', True),
    ('web-dom', 'Web · DOM', False),
]
LOCALES = [('en', 'English'), ('fr', 'Français'), ('ar', 'العربية'),
           ('de', 'Deutsch'), ('ja', '日本語'), ('zh-CN', '简体中文'), ('ko', '한국어')]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def build(output):
    output.mkdir(parents=True, exist_ok=True)
    previous_path = output / 'manifest.json'
    previous = json.loads(previous_path.read_text()) if previous_path.exists() else {}
    previous_captures = {c['id']: c for c in previous.get('captures', [])}
    resources = APP / 'scripts/review'
    strings = json.loads((resources / 'strings.en.json').read_text())
    source_files = [APP / 'Cargo.toml', APP / 'Cargo.lock', APP / 'Day.toml', APP / 'build.rs']
    for folder in ('src', 'resource', 'dayscript'):
        source_files.extend(p for p in (APP / folder).rglob('*') if p.is_file())
    hashes = {str(p.relative_to(APP)): digest(p) for p in sorted(source_files) if p.exists()}
    source_hash = hashlib.sha256(json.dumps(hashes, sort_keys=True).encode()).hexdigest()
    issue_file = APP / 'build/capture-issues.json'
    issues = json.loads(issue_file.read_text()) if issue_file.exists() else {}
    captures, actual, expected = [], 0, 0
    for target, platform, mobile in TARGETS:
        for locale, language in LOCALES:
            for theme in ('light', 'dark'):
                variant = theme + ('' if locale == 'en' else '-' + locale)
                item = dict(id=f'{target}/{variant}', target=target, platform=platform,
                            locale=locale, language=language, theme=theme, mobile=mobile,
                            images=[], issue=issues.get(target, ''))
                positions = ('top', 'bottom') if mobile else ('overview',)
                for position in positions:
                    expected += 1
                    relative = Path('screenshots') / target / variant / f'{position}.png'
                    source = APP / 'build/day' / relative
                    if not source.exists():
                        continue
                    raw = source.read_bytes()
                    if raw[:8] != b'\x89PNG\r\n\x1a\n':
                        raise ValueError(f'Invalid PNG: {source}')
                    width, height = struct.unpack('>II', raw[16:24])
                    if min(width, height) < 100:
                        raise ValueError(f'Unexpectedly small screenshot: {source}')
                    dest = output / relative
                    dest.parent.mkdir(parents=True, exist_ok=True)
                    shutil.copy2(source, dest)
                    item['images'].append(dict(position=position, path=relative.as_posix(),
                                               width=width, height=height, sha256=digest(source)))
                    actual += 1
                item['complete'] = len(item['images']) == len(positions)
                if mobile and item['complete'] and item['images'][0]['sha256'] == item['images'][1]['sha256']:
                    raise ValueError(f'Top and bottom captures are identical: {item["id"]}')
                prior = previous_captures.get(item['id'], {})
                same_images = item['images'] and item['images'] == prior.get('images')
                item['sourceHash'] = prior.get('sourceHash', previous.get('sourceHash', source_hash)) if same_images else source_hash
                captures.append(item)
    data = dict(schema=1, sourceHash=source_hash, sourceFiles=hashes,
                createdAt=datetime.now(timezone.utc).isoformat(), actual=actual,
                expected=expected, strings=strings, captures=captures)
    data['sourceSnapshots'] = previous.get('sourceSnapshots', {})
    if previous.get('sourceHash') and previous.get('sourceFiles'):
        data['sourceSnapshots'][previous['sourceHash']] = previous['sourceFiles']
    data['sourceSnapshots'][source_hash] = hashes
    data['previousSourceHashes'] = sorted(set(previous.get('previousSourceHashes', []) + ([previous['sourceHash']] if 'sourceHash' in previous else [])) - {source_hash})
    provenance = APP / 'build/capture-provenance.json'
    if provenance.exists():
        data['provenance'] = json.loads(provenance.read_text())
    validation = APP / 'build/validation.json'
    if validation.exists():
        shutil.copy2(validation, output / 'validation.json')
    logs = APP / 'build/capture-logs'
    if logs.exists():
        shutil.copytree(logs, output / 'capture-logs', dirs_exist_ok=True)
    (output / 'manifest.json').write_text(json.dumps(data, ensure_ascii=False, indent=2) + '\n')
    esc = html.escape
    def text(key):
        return esc(strings[key])
    embedded = json.dumps(data, ensure_ascii=False).replace('<', '\\u003c')
    page = f'''<!doctype html>
<html lang="en"><meta charset="utf-8"><meta name="viewport" content="width=device-width,initial-scale=1">
<title>{text('title')}</title><style>{(resources / 'review.css').read_text()}</style>
<header><h1>{text('heading')}</h1><p>{text('intro')}</p></header>
<main>
<section class="preview" aria-label="{text('preview')}">
<div id="themes" class="dash-group themes" role="group" aria-label="{text('theme')}"></div>
<div id="stage" class="stage" aria-label="{text('preview')}" aria-busy="true"></div>
<div id="platforms" class="dash-group platforms" role="group" aria-label="{text('platform')}"></div>
<div id="languages" class="dash-group languages" role="group" aria-label="{text('language')}"></div>
</section>
<div class="caption" aria-live="polite" aria-atomic="true"><h2 id="current-platform"></h2><p id="current-detail"></p></div>
<details class="review-panel"><summary>{text('review_controls')}</summary><div id="review"></div>
<div class="actions"><button id="export">{text('export')}</button><span id="summary"></span></div><p id="storage"></p></details>
</main><footer><p>{text('footer')}</p><a href="manifest.json">{text('manifest')}</a></footer>
<script>const DATA={embedded};\n{(resources / 'review.js').read_text()}</script></html>'''
    (output / 'index.html').write_text(page)
    print(f'{actual}/{expected} screenshots; {len(captures)} capture sets: {output / "index.html"}')


if __name__ == '__main__':
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--output', type=Path, required=True)
    build(parser.parse_args().output.resolve())
