// Website renditions only. The capture tree and CI's original PNG artifacts stay untouched.
import { createHash } from 'node:crypto';
import { readdir, readFile, writeFile, unlink, stat } from 'node:fs/promises';
import { join, relative, sep } from 'node:path';

const PREVIEW_BYTES = 120_000;

async function files(root) {
  const entries = await readdir(root, { recursive: true, withFileTypes: true });
  return entries.filter(e => e.isFile()).map(e => join(e.parentPath, e.name));
}

export async function preparePreviews(publicDir, siteDir, sharp) {
  const publicFiles = await files(publicDir);
  const captures = publicFiles.filter(p => relative(publicDir, p).split(sep).includes('gallery') && p.endsWith('.png'));
  const replacements = new Map();
  let before = 0, after = 0;
  // A small pool keeps thousands of high-resolution captures from exhausting a CI runner.
  const queue = captures.slice();
  await Promise.all(Array.from({ length: 3 }, async () => {
    for (let path = queue.shift(); path; path = queue.shift()) {
      const source = await readFile(path);
      let width = 1280, height = 1440, result;
      for (;;) {
        result = await sharp(source).resize({ width, height, fit: 'inside', withoutEnlargement: true })
          .webp({ quality: 82 }).toBuffer({ resolveWithObject: true });
        if (result.data.length <= PREVIEW_BYTES) break;
        if (Math.min(width, height) < 400) throw new Error(`Cannot fit readable preview in byte budget: ${path}`);
        width = Math.floor(width * 0.85);
        height = Math.floor(height * 0.85);
      }
      const output = path.slice(0, -4) + '.webp';
      await writeFile(output, result.data);
      const oldPath = relative(publicDir, path).split(sep).join('/');
      replacements.set(oldPath, {
        path: oldPath.slice(0, -4) + '.webp', width: result.info.width, height: result.info.height,
        bytes: result.data.length, sha256: createHash('sha256').update(result.data).digest('hex'),
      });
      before += source.length;
      after += result.data.length;
    }
  }));
  const match = value => {
    if (typeof value !== 'string' || !value.endsWith('.png')) return undefined;
    for (const [path, replacement] of replacements) {
      if (value === path || value.endsWith('/' + path)) return { path, replacement };
    }
  };
  const rewrite = value => {
    if (typeof value === 'string') {
      const found = match(value);
      return found ? value.slice(0, -found.path.length) + found.replacement.path : value;
    }
    if (Array.isArray(value)) return value.map(rewrite);
    if (!value || typeof value !== 'object') return value;
    const found = match(value.path) ?? match(value.src) ?? match(value.url);
    const out = Object.fromEntries(Object.entries(value).map(([k, v]) => [k, rewrite(v)]));
    if (found) {
      for (const field of ['width', 'height', 'bytes', 'sha256']) {
        if (field in out) out[field] = found.replacement[field];
      }
      if (typeof out.file === 'string') out.file = found.replacement.path.split('/').at(-1);
    }
    return out;
  };
  // Update both the public machine-readable index and the inputs Astro uses for every locale.
  const dataFiles = new Set([...publicFiles, ...await files(siteDir)]);
  for (const path of dataFiles) {
    if (!path.endsWith('.json') || path.split(sep).includes('node_modules')) continue;
    const original = await readFile(path, 'utf8');
    const parsed = JSON.parse(original);
    const updated = rewrite(parsed);
    if (JSON.stringify(updated) !== JSON.stringify(parsed)) {
      await writeFile(path, JSON.stringify(updated, null, 2) + '\n');
    }
  }
  // Remove only the staged PNG copies, after every manifest points to a valid WebP.
  await Promise.all(captures.map(path => unlink(path)));
  console.log(`Website previews: ${captures.length} captures, ${(before / 1e6).toFixed(1)} MB → ${(after / 1e6).toFixed(1)} MB; original artifacts unchanged.`);
  return { count: captures.length, before, after };
}

export function siteSizeCheck() {
  return {
    name: 'day-travel-pages-size',
    hooks: {
      'astro:build:done': async ({ dir, logger }) => {
        const { fileURLToPath } = await import('node:url');
        const paths = await files(fileURLToPath(dir));
        let bytes = 0;
        for (const path of paths) bytes += (await stat(path)).size;
        logger.info(`Complete site: ${(bytes / 1e6).toFixed(1)} MB`);
        if (bytes > 950_000_000) throw new Error('Site exceeds the 950 MB publication budget; GitHub Pages allows 1 GB.');
      },
    },
  };
}
