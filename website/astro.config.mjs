import { createRequire } from 'node:module';
import { dirname, join } from 'node:path';
import { fileURLToPath } from 'node:url';
import { preparePreviews, siteSizeCheck } from './preview-images.mjs';

// The shared workflow supplies the checked-out template and its pinned dependencies.
const template = process.env.DAYSITE_ROOT;
if (!template) throw new Error('Build through daysite/scripts/build-site.mjs so DAYSITE_ROOT is set.');
const require = createRequire(join(template, 'package.json'));
const publicDir = process.env.DAYSITE_PUBLIC_DIR ?? join(template, 'public');
const siteConfig = process.env.DAYSITE_CONFIG ?? fileURLToPath(new URL('./site.toml', import.meta.url));
await preparePreviews(publicDir, dirname(siteConfig), require('sharp'));
const { createDaysiteConfig } = await import(`${template}/src/config.mjs`);
const config = await createDaysiteConfig({
  root: fileURLToPath(new URL('./', import.meta.url)),
  siteConfig, publicDir, outDir: process.env.DAYSITE_OUT_DIR,
});
config.integrations.push(siteSizeCheck());
export default config;
