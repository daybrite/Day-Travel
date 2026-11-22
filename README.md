# Day Travel

A native city browser and travel planner built with Day. Choose one of 28 destinations from a native desktop sidebar, a searchable phone list, or a map pin. Full-width photographs are bundled from the supplied TravelBookings catalog. Each city page combines native planning controls with a compact city map. Saving captures the departure date, nights, travelers, ticket class, interests and round-trip choice in a session snapshot.

City pages fit the viewport without page scrolling. Save my trip sits beside the city heading,
and the map expands or shrinks into the space left below the photo and planning controls.
Planning controls use aligned, labeled form rows, including a native segmented ticket-class
picker and a Round-trip toggle. Saved trips preserve these selections; one-way trips omit
the return date.
The map has no visible caption and fills the page width and remaining height. Native phone navigation containers still reserve the bottom system safe area.

The home page includes an interactive map below the city list; each destination has a separate map centered on its coordinates. macOS/AppKit and iOS/UIKit use Apple MapKit, Android/Material uses the Google Maps SDK, and other targets use Day’s canvas map renderer with OpenStreetMap data from OpenFreeMap. Maps require network access; browsing photos and planning trips work offline. No location permission is needed. Map-provider dependencies use portable version requirements; local development can override them with `day patch --local ../day-piece-map --check`.
The app supports US English, French, German, Japanese, Simplified Chinese, Korean, and Arabic. All platforms use the same trip data. Translations are draft demo content for review before publication.

## Run locally

Install a Day CLI from `main`, then patch this app's dependencies to the same branch:

```sh
cargo install --git https://github.com/daybrite/day --branch main day-cli --locked
day patch --main --check
day launch -t macos-appkit --detach --locale en
```

The manifest uses published Day version requirements. `day patch` writes an ignored `.cargo/config.toml` that selects current development commits. For framework development, use `day patch --local ../day --check` and the CLI from that checkout instead.

Before building Android, run `python3 scripts/configure_google_maps.py`. It uses `DAY_GOOGLE_MAPS_API_KEY` when set, otherwise checks Day-Showcase's manifest and then the map demo's development key (approved for this local preview). The credential is written to the ignored `platform/android/res/values/maps-local.xml` resource and is never printed. For another installation, configure a Maps SDK for Android key authorized for `dev.daybrite.travel` and its signing certificate. Apple and canvas maps need no key.

CI passes the repository's `DAY_GOOGLE_MAPS_API_KEY` secret to Android's setup command. It must be available for Google Maps captures; fork pull requests do not receive repository secrets. The shared `daybrite/actions` workflow must support forwarding this optional service key to target setup.

The language menu updates translations and formatting immediately. Layout direction is fixed at launch: launch with `--locale ar` to see the mirrored Arabic interface. A localized note explains when a language change needs a restart. The sun/moon button switches appearance where the toolkit supports changing it at runtime. `--themes light,dark` launches each appearance independently for screenshots.

The departure picker is native on AppKit, UIKit and Material. The traveler stepper uses the platform’s numeric control where supported and native buttons plus a text field elsewhere. The return date follows the selected departure date and nights across month/year boundaries. Saving captures all choices; editing retains the root-owned draft. The planner is an inspiration demo, not a booking service.

City source data, photo download hashes and translations are documented in `data/travelbookings/README.md`. App-owned text uses generated Fluent accessors; country names remain provider data. City captions are reviewed and localized separately; [CAPTIONS.md](data/travelbookings/CAPTIONS.md) records all 28 sources and distinguishes nicknames, UNESCO designations, and factual descriptions.
## Screenshots

The generated destination scripts visit all 28 city pages in source order. `dayscript/demo.yaml` captures the browser, embedded map, city page, edited controls and saved itinerary while verifying interactions. Screenshots capture actual native windows or mobile viewports.

```sh
scripts/capture.sh macos-appkit
scripts/capture.sh ios-uikit --ios-simulator <simulator-UDID>
ANDROID_SERIAL=emulator-5554 scripts/capture.sh android-material --android-device emulator-5554
```

Set `DAY_CLI` to override the CLI path. Extra arguments pass through to `day launch`; use `--skip-build` only when the binary matches the source. Native SDKs and the selected emulator or simulator must already be installed.

For Android captures, use the CLI and app built from current Day `main`; older CLIs can
label a capture “light” without changing the native controls. A previously selected
in-app appearance can also persist on a development emulator. Use a fresh demo
installation for reproducible captures, as CI does.

The destination capture scripts assert the translated city heading, photo, map, and planning controls, wait for layout, then capture. Each variant starts a fresh app so state and layout direction are reproducible. Desktop dimensions are set in `Day.toml`. Output is under `build/day/screenshots/<target>/<variant>/` (named devices add a directory beneath the target).

Both capture wrappers and CI run `scripts/ci/check-screenshots.py`. It requires every city in every shipped locale and both themes, checks PNG headers, dimensions and completion, rejects identical light/dark pairs, and writes a checksum coverage report beside the captures. A missing variant fails the run rather than silently producing a partial gallery.

The current matrix includes AppKit, GTK, and Qt on macOS; UIKit on iOS; Material on Android; GTK and Qt on Linux; WinUI and legacy XAML on Windows; ArkUI on HarmonyOS; and the browser DOM target. The legacy Windows target is included for comparison.

Host-specific capture notes:

- macOS GTK: set `PANGOCAIRO_BACKEND=fc` for complete Japanese font fallback with this host’s Homebrew Pango build.
- Linux Qt on `skipbox.local`: use `QT_QPA_PLATFORM=xcb QT_QPA_PLATFORMTHEME=gtk3 QT_STYLE_OVERRIDE=Fusion` under an isolated Xvfb/D-Bus session. Capture light and dark in separate runs, with `GTK_THEME=Adwaita` and `GTK_THEME=Adwaita:dark` respectively; the host’s Qt platform plugin otherwise ignores the requested dark scheme. An isolated `XDG_CONFIG_HOME` with `gtk-3.0/settings.ini` selecting `gtk-icon-theme-name=breeze` supplies the expected add/remove glyphs.
- Linux GTK on `skipbox.local`: use `GDK_BACKEND=x11 GSK_RENDERER=cairo xvfb-run -a -s '-screen 0 2600x2200x24' dbus-run-session -- scripts/capture.sh linux-gtk`. This isolates the display and D-Bus session and avoids a stalled GPU render checkpoint on this host.
- HarmonyOS: use a separate portrait phone emulator and set `DAY_OHOS_TARGET=<hdc-address>` as well as `--ohos-device <hdc-address>` so device-level screenshots use the same emulator. The review used a 360 × 720 viewport on `skipbox.local`. Its minimal image lacked the Noto Sans CJK font referenced by its font configuration; the existing Linux Noto Sans CJK package was installed into this separate emulator’s `/system/fonts/` and the emulator restarted to enable Korean glyphs.
- Windows: run `scripts/capture.ps1 -Target windows-winui -SmallBuild` (or `windows-xaml`) in the logged-in desktop session. An SSH shell alone may not have an interactive display. When copying macOS sources, exclude AppleDouble `._*` metadata files. On the space-constrained review VM, debug symbols and incremental compilation were disabled, and each target’s generated Cargo cache was removed after its screenshots were saved.
- Web: set `DAY_WEB_DRIVER` to the framework's `node .../day-cli/resources/web/webdom-driver.mjs`, `DAY_WEB_DRIVER_PLAYWRIGHT` to the installed Playwright package directory, and `DAY_WEB_DRIVER_BROWSER=chromium`.

## Legacy localization review gallery

The gallery below is retained for the previous two-screen localization review. Current city-by-city captures and navigation walkthrough screenshots are available directly under `build/day/screenshots/`.

Collect each host's target folders into `build/day/screenshots/`, then run:

```sh
python3 scripts/build_review.py --output "$HOME/Desktop/Day-Travel-Review"
open -a Safari "$HOME/Desktop/Day-Travel-Review/index.html"
```

The gallery works offline. Hover over or tap the dashes to select appearance above the image, platform on the right, and language below. Each selection preserves the other two; arrow keys also move between choices. Mobile captures appear in pairs. Expand **Review and export decisions** to approve the selected capture or add notes. Approvals and notes persist in the browser; **Export decisions** saves a portable JSON file. An exported file can be supplied for a later editing session. The page does not upload decisions or publish screenshots.

`manifest.json` records dimensions, screenshot checksums, and app source hashes. Capture logs and `validation.json` accompany this review build. Refreshing the gallery preserves notes; changed screenshot checksums clear only the affected approval. Missing captures remain visible and cannot be approved. The builder rejects invalid or tiny PNGs and identical top/bottom pairs. It does not substitute an image from another target. Optional `build/capture-issues.json` maps target identifiers to diagnostic notes.

Review UI strings live in `scripts/review/strings.en.json`; the builder and generated JavaScript accessors supply all UI copy from that catalog.

## Check and edit

```sh
cargo fmt --all -- --check
cargo test
cargo clippy --all-targets -- -D warnings
day lint --allow store-missing-field
day launch -t macos-appkit --locale en --script dayscript/desktop.yaml
day launch -t ios-uikit --locale en-US --script dayscript/demo.yaml
```

The store-field waiver covers missing privacy-policy URLs for this unpublished sample. `demo.yaml` exercises city search, stack navigation, map controls, date changes, traveler counts, interests, saving/editing, and language selection; capture scripts preserve the selected locale instead.

- `src/lib.rs`: screen layout, appearance, and interactions.
- `resource/locales/<locale>/app.ftl`: translated UI and accessibility labels.
- `resource/images/photo_*.jpg`: bundled city photographs, accessed through generated resource symbols.
- `data/travelbookings/`: source catalog, translated city names, and photo provenance.
- `dayscript/`: interaction checks and capture walkthroughs.
- `scripts/`: capture wrapper and portable review gallery.

## CI and project website

`.github/workflows/ci.yml` uses the standard `daybrite/actions` app workflow. It runs on pushes to `main`, pull requests, version tags, manual dispatch, and nightly at 22:30 America/New_York. `day-deps: main` resolves Day's main branch once per run; every target uses that commit for both its dependencies and CLI.

CI captures all **28 cities × 7 locales × 2 themes × 10 targets = 3,920 PNGs** (392 per target). The shared workflow does not support legacy `windows-xaml`; it remains available locally through the same 392-image capture wrapper. iOS, Android, and HarmonyOS each use one portrait phone. Every destination gets one viewport image. The completeness check runs before artifact upload and website publication. Capture failures block publication.

Capture scripts wait for the map and assert that the planning controls are visible.
For canvas maps, readiness requires completed tile requests and a rendered basemap,
so a loaded style with an empty map does not count as ready. The coverage validator
checks every expected city, locale, and theme, validates PNG headers and dimensions,
and rejects identical light/dark files. Visual inspection is still needed for layout
quality and correct native appearance; file counts and checksums cannot prove those.

The main-branch workflow publishes the [project website](https://daybrite.github.io/Day-Travel/), including its screenshot gallery and browser app. GitHub Pages is already configured to deploy through Actions. `website/site.toml` configures the standard daysite template; site copy comes from `store/storefront.toml`. Pull requests validate the app without publishing the website. Generated screenshots and site files stay out of Git.

The website uses smaller WebP previews; CI artifacts retain every original PNG at capture
resolution. `website/astro.config.mjs` prepares the staged gallery before Astro renders it,
updating image URLs, dimensions, sizes, and checksums. Previews fit within 1280 × 1440 pixels
and 120 KB each, shrinking further only when needed. The finished site must stay below
950 MB to leave headroom under GitHub Pages' 1 GB limit. Run the preview regression check
with `DAYSITE_ROOT=/path/to/daysite node --test website/preview-images.test.mjs`.

Font setup uses the shared workflow's existing hooks:

- Linux and browser runners already receive `fonts-noto-cjk` from the shared setup.
- `scripts/ci/setup-captures.sh` selects Fontconfig for macOS GTK so Japanese fallback works.
- HarmonyOS installs the host Noto CJK package, copies its fallback font into the disposable CI emulator, and reboots before capturing. The helper requires a loopback device and emulator images under `RUNNER_TEMP`; it refuses ordinary local devices.

The workflow runs normal lint checks with only the sample's missing store privacy-policy URLs waived. It does not submit to app stores. Publishing this project site does not add the screenshots to the separate daybrite.dev landing page; that integration is still to come.

Desktop targets use a native sidebar for all 28 cities, Discover, My trip, and Settings.
Phone targets use a navigation stack. Discover and city photos extend behind the status
and navigation bars on iOS and Android, retaining native Back controls without a repeated city title. Pages use platform backgrounds and semantic text colors. This uses Day's
`NavStack::immersive` page predicate and Android's `edge-to-edge` manifest setting.
Run `dayscript/desktop.yaml` on desktop and `dayscript/demo.yaml` on mobile.
