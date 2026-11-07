// UI copy is generated from strings.en.json. Capture titles are publication metadata.
const res = {str: Object.fromEntries(Object.entries(DATA.strings).map(([k, v]) =>
  [k, (args = {}) => v.replace(/\{(\w+)\}/g, (_, n) => String(args[n] ?? ''))]))};
// Generated vector accessors match the string accessors; trusted bundled SVGs only.
res.vectors = Object.fromEntries(Object.entries(DATA.vectors).map(([name, markup]) => [name, () => {
  const template = document.createElement('template');
  template.innerHTML = markup;
  const icon = template.content.firstElementChild;
  icon.classList.add('control-icon');
  icon.setAttribute('aria-hidden', 'true');
  icon.setAttribute('focusable', 'false');
  return icon;
}]));
const platformIcons = {
  'macos-appkit': res.vectors.macos_appkit,
  'ios-uikit': res.vectors.ios_uikit,
  'android-material': res.vectors.android_material,
  'linux-gtk': res.vectors.linux_gtk,
  'linux-qt': res.vectors.linux_qt,
  'windows-winui': res.vectors.windows_winui,
  'harmony-arkui': res.vectors.harmony_arkui,
  'web-dom': res.vectors.web_dom,
};
const q = id => document.getElementById(id);
function el(tag, cls, text) {
  const node = document.createElement(tag);
  if (cls) node.className = cls;
  if (text !== undefined) node.textContent = text;
  return node;
}
// Keep the existing storage key: rebuilding the gallery must preserve review notes.
// Approvals apply to exact PNG checksums; changing only the preview never invalidates them.
const key = 'day-travel-review:v1:' + location.pathname;
let decisions = {}, storageWorks = true;
try {
  let saved = localStorage.getItem(key);
  if (!saved) for (const hash of [DATA.sourceHash, ...(DATA.previousSourceHashes || [])]) {
    saved = localStorage.getItem('day-travel-review:' + hash);
    if (saved) break;
  }
  decisions = JSON.parse(saved || '{}');
  for (const c of DATA.captures) {
    const d = decisions[c.id];
    if (d && JSON.stringify(d.images) !== JSON.stringify(checksums(c))) d.approved = false;
  }
} catch { storageWorks = false; }
function checksums(c) { return c.images.map(s => ({path: s.path, sha256: s.sha256})); }
function summary() {
  q('summary').textContent = res.str.summary({actual: DATA.actual, expected: DATA.expected,
    approved: DATA.captures.filter(c => c.complete && decisions[c.id]?.approved).length});
  q('storage').textContent = storageWorks ? res.str.saved() : res.str.nosave();
}
function save() {
  try { localStorage.setItem(key, JSON.stringify(decisions)); } catch { storageWorks = false; }
  summary();
}
const axes = [
  {id: 'themes', field: 'theme', options: ['light', 'dark'].map(v => [v, res.str[v]()])},
  {id: 'platforms', field: 'target', options: [...new Map(DATA.captures.map(c => [c.target, c.platform]))]},
  {id: 'languages', field: 'locale', options: [...new Map(DATA.captures.map(c => [c.locale, c.language]))]},
];
const selection = {target: DATA.captures[0].target, locale: 'en', theme: 'light'};
try {
  const saved = JSON.parse(localStorage.getItem(key + ':selection') || '{}');
  for (const axis of axes) if (axis.options.some(([v]) => v === saved[axis.field])) selection[axis.field] = saved[axis.field];
} catch { /* Selection persistence is optional; the preview still works without storage. */ }
let hoverTimer;
for (const axis of axes) {
  const group = q(axis.id);
  for (const [value, label] of axis.options) {
    const button = el('button', 'dash');
    button.type = 'button';
    button.dataset.value = value;
    button.setAttribute('aria-label', label);
    button.setAttribute('aria-controls', 'stage');
    const bar = el('span', 'bar'), tip = el('span', 'tip', label);
    bar.setAttribute('aria-hidden', 'true'); tip.setAttribute('aria-hidden', 'true');
    if (axis.field === 'theme') {
      button.append(value === 'light' ? res.vectors.sun() : res.vectors.moon(), bar);
    } else if (axis.field === 'target') {
      button.dataset.pf = value;
      button.append(bar, platformIcons[value]());
    } else {
      const name = el('span', 'locale-name', label);
      name.lang = value;
      name.dir = 'auto';
      name.setAttribute('aria-hidden', 'true');
      button.append(bar, name);
    }
    if (axis.field !== 'locale') button.append(tip);
    const select = () => {
      clearTimeout(hoverTimer);
      if (selection[axis.field] === value) return;
      selection[axis.field] = value;
      try { localStorage.setItem(key + ':selection', JSON.stringify(selection)); } catch { /* optional */ }
      render();
    };
    // Touch pointerenter is not a hover: tapping still uses the normal click handler.
    button.addEventListener('pointerenter', event => {
      if (event.pointerType === 'mouse') hoverTimer = setTimeout(select, 90);
    });
    button.addEventListener('pointerleave', () => clearTimeout(hoverTimer));
    button.addEventListener('click', select);
    button.addEventListener('focus', select);
    group.append(button);
  }
  group.addEventListener('keydown', event => {
    const buttons = [...group.children], index = buttons.indexOf(document.activeElement);
    if (index < 0) return;
    const delta = ['ArrowRight', 'ArrowDown'].includes(event.key) ? 1 : ['ArrowLeft', 'ArrowUp'].includes(event.key) ? -1 : 0;
    const next = event.key === 'Home' ? 0 : event.key === 'End' ? buttons.length - 1 : delta ? (index + delta + buttons.length) % buttons.length : null;
    if (next !== null) { event.preventDefault(); buttons[next].focus(); }
  });
}
let revision = 0;
async function render() {
  const request = ++revision;
  for (const axis of axes) for (const button of q(axis.id).children)
    button.setAttribute('aria-pressed', String(button.dataset.value === selection[axis.field]));
  const c = DATA.captures.find(c => c.target === selection.target && c.locale === selection.locale && c.theme === selection.theme);
  q('stage').setAttribute('aria-busy', 'true');
  // Do not let an approval land on the preceding capture while the next is decoding.
  for (const input of q('review').querySelectorAll('input, textarea')) input.disabled = true;
  const slide = el('div', 'slide'), images = el('div', 'images' + (c.mobile ? ' mobile' : ''));
  slide.dataset.capture = c.id;
  const pending = [];
  for (const shot of c.images) {
    const figure = el('figure'), link = el('a'), img = el('img');
    figure.style.setProperty('--shot-ratio', shot.width / shot.height);
    link.href = shot.path; link.target = '_blank'; link.rel = 'noopener';
    img.alt = [c.platform, c.language, res.str[c.theme](), res.str[shot.position]()].join(' · ');
    img.width = shot.width; img.height = shot.height; img.src = shot.path;
    // Decode before crossfading. A request token prevents fast hovering from showing
    // a slow, stale selection after a newer selection has already finished loading.
    pending.push(img.decode());
    link.append(img); figure.append(link, el('figcaption', '', res.str[shot.position]())); images.append(figure);
  }
  slide.append(images);
  const loaded = (await Promise.allSettled(pending)).every(result => result.status === 'fulfilled');
  if (request !== revision) return;
  if (!loaded) slide.replaceChildren(el('p', 'missing', res.str.load_error()));
  else if (!c.complete) slide.append(el('p', 'missing', res.str.missing() + (c.issue ? ' — ' + c.issue : '')));
  const old = [...q('stage').children];
  // Each slide includes its opaque themed background. Fade the new composition
  // OVER the old one; fading both out/in exposes the stage and causes a flash.
  // Let interrupted fades finish underneath the next one, preserving their visible
  // composite. Remove old layers only when an opaque replacement has finished.
  for (const node of old) { node.inert = true; node.setAttribute('aria-hidden', 'true'); }
  slide.dataset.theme = c.theme;
  slide.classList.add('is-visible');
  q('stage').append(slide);
  q('stage').dataset.theme = c.theme;
  const reducedMotion = matchMedia('(prefers-reduced-motion: reduce)').matches;
  if (old.length && !reducedMotion) {
    const fade = slide.animate([{opacity: 0}, {opacity: 1}], {
      duration: 480, easing: 'cubic-bezier(.4, 0, .2, 1)',
    });
    fade.finished.then(() => old.forEach(node => node.remove()));
  } else old.forEach(node => node.remove());
  q('stage').setAttribute('aria-busy', 'false');
  q('current-platform').textContent = c.platform;
  q('current-detail').textContent = c.language + ' · ' + res.str[c.theme]();
  renderReview(c, loaded);
  summary();
}
function renderReview(c, loaded) {
  const d = decisions[c.id] || {}, review = el('div', 'review');
  const label = el('label', 'approve'), input = el('input');
  input.type = 'checkbox'; input.checked = Boolean(d.approved); input.disabled = !c.complete || !loaded;
  label.append(input, document.createTextNode(res.str.approve()));
  const field = el('label', 'field', res.str.notes()), note = el('textarea');
  note.value = d.notes || ''; field.append(note); review.append(label, field);
  const update = () => {
    decisions[c.id] = {approved: input.checked, notes: note.value, updatedAt: new Date().toISOString(), images: checksums(c)};
    save();
  };
  input.addEventListener('change', update); note.addEventListener('input', update);
  q('review').replaceChildren(review);
}
q('export').addEventListener('click', () => {
  const data = {schema: 1, sourceHash: DATA.sourceHash, exportedAt: new Date().toISOString(), decisions};
  const url = URL.createObjectURL(new Blob([JSON.stringify(data, null, 2)], {type: 'application/json'}));
  const link = el('a'); link.href = url; link.download = 'day-travel-review-decisions.json'; link.click();
  setTimeout(() => URL.revokeObjectURL(url), 1000);
});
render();
