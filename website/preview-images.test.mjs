import assert from 'node:assert/strict';
import { createHash } from 'node:crypto';
import { mkdtemp, mkdir, readFile, writeFile, rm, access, truncate } from 'node:fs/promises';
import { createRequire } from 'node:module';
import { tmpdir } from 'node:os';
import { join } from 'node:path';
import { pathToFileURL } from 'node:url';
import test from 'node:test';
import { preparePreviews, siteSizeCheck } from './preview-images.mjs';

const sharp = createRequire(join(process.env.DAYSITE_ROOT, 'package.json'))('sharp');

test('website previews preserve originals and update all image references and metadata', async t => {
  const root = await mkdtemp(join(tmpdir(), 'travel-preview-test-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  const pub = join(root, 'public'), site = join(root, 'site');
  const path = 'gallery/android-material/phone/light-de/fixture.png';
  await mkdir(join(pub, 'gallery/android-material/phone/light-de'), { recursive: true });
  await mkdir(site);
  // Synthetic image fixture, separate from app-owned resources and UI translations.
  const original = await sharp({ create: { width: 1800, height: 2400, channels: 3, background: '#287eae' } }).png().toBuffer();
  await writeFile(join(root, 'original.png'), original);
  await writeFile(join(pub, path), original);
  await writeFile(join(pub, 'icon.png'), original);
  await writeFile(join(pub, 'gallery/gallery.json'), JSON.stringify({ screenshots: [{
    path, url: 'https://example.test/Day-Travel/' + path, file: 'fixture.png',
    width: 1800, height: 2400, bytes: original.length, sha256: 'old',
  }] }));
  await writeFile(join(site, 'gallery-manifest.json'), JSON.stringify({ image: { src: path, width: 1800, height: 2400 } }));
  await writeFile(join(site, 'appindex.json'), JSON.stringify({ screenshots: [path] }));
  const result = await preparePreviews(pub, site, sharp);
  assert.equal(result.count, 1);
  assert.deepEqual(await readFile(join(root, 'original.png')), original);
  assert.deepEqual(await readFile(join(pub, 'icon.png')), original);
  await assert.rejects(access(join(pub, path)));
  const index = JSON.parse(await readFile(join(pub, 'gallery/gallery.json'), 'utf8')).screenshots[0];
  const image = await readFile(join(pub, index.path));
  const metadata = await sharp(image).metadata();
  assert.equal(index.file, 'fixture.webp');
  assert.equal(index.url, 'https://example.test/Day-Travel/' + index.path);
  assert.equal(index.bytes, image.length);
  assert.ok(image.length <= 120_000);
  assert.equal(index.sha256, createHash('sha256').update(image).digest('hex'));
  assert.equal(index.width, metadata.width);
  assert.equal(index.height, metadata.height);
  assert.ok(metadata.width <= 1280 && metadata.height <= 1440);
  assert.equal(JSON.parse(await readFile(join(site, 'gallery-manifest.json'), 'utf8')).image.src, index.path);
  assert.deepEqual(JSON.parse(await readFile(join(site, 'appindex.json'), 'utf8')).screenshots, [index.path]);
  // A repeated local preview keeps the already prepared renditions and valid manifests.
  assert.equal((await preparePreviews(pub, site, sharp)).count, 0);
});

test('publication budget counts downloads as well as preview images', async t => {
  const root = await mkdtemp(join(tmpdir(), 'travel-size-test-'));
  t.after(() => rm(root, { recursive: true, force: true }));
  const file = join(root, 'synthetic-download.bin');
  await writeFile(file, '');
  // Sparse fixture: exercise the size guard without allocating a large payload.
  await truncate(file, 951_000_000);
  const check = siteSizeCheck().hooks['astro:build:done'];
  const context = { dir: pathToFileURL(root), logger: { info() {} } };
  await assert.rejects(check(context), /950 MB/);
  await truncate(file, 900_000_000);
  await check(context);
});
