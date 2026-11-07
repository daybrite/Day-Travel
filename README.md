# Day Travel

A small native Day app for demonstrating localization on daybrite.dev. Its fictional weekend in Valletta, Malta includes translated text, locale-aware dates and numbers, plural forms, right-to-left layout, and light and dark appearances. The harbour illustrations are original SVGs. Windows uses 2× PNGs generated from those SVGs: its current gradient fallback pads illustrations to a square. Run `scripts/render_windows_art.sh` after editing the artwork (requires `resvg`). Other targets keep the vectors.

The app supports English, French, Arabic, German, Japanese, Simplified Chinese, and Korean. All platforms use the same trip data. Translations are draft demo content for review before publication.

## Run locally

Keep this project beside the `day/` framework checkout. It currently uses local Cargo path dependencies; no repository or package has been published.

```sh
../day/target/debug/day launch -t macos-appkit --detach --locale en
```

Use the matching CLI from the framework checkout. An older globally installed CLI may have different target names.

The language menu updates translations and formatting immediately. Layout direction is fixed at launch: launch with `--locale ar` to see the mirrored Arabic interface. A localized note explains when a language change needs a restart. The sun/moon button switches appearance where the toolkit supports changing it at runtime. `--themes light,dark` launches each appearance independently for screenshots.

The screen stacks its columns on narrow windows. Traveler buttons update plural forms and the per-person budget; packing switches update the packed count. The trip budget remains EUR 1,284.50 in every locale. Fluent `NUMBER` formats numbers and `DATETIME` formats fixed civil dates and times; this demo does not convert currencies or time zones.

## Screenshots

Each target gets fourteen capture sets: seven languages in two appearances. Desktop targets capture one complete window. Mobile targets capture the actual viewport at the top and bottom of the scroll view, for side-by-side display. These are separate screenshots, not a stitched image.

```sh
scripts/capture.sh macos-appkit
scripts/capture.sh ios-uikit --ios-device <simulator-UDID>
ANDROID_SERIAL=emulator-5554 scripts/capture.sh android-material --android-device emulator-5554
```

Set `DAY_CLI` to override the CLI path. Extra arguments pass through to `day launch`; use `--skip-build` only when the binary matches the source. Native SDKs and the selected emulator or simulator must already be installed.

The capture scripts assert the translated trip heading and initial traveler count, wait for layout, then capture. Each variant starts a fresh app so state and layout direction are reproducible. Desktop dimensions are set in `Day.toml`. Output is under `build/day/screenshots/<target>/<variant>/`.

The current matrix includes AppKit, GTK, and Qt on macOS; UIKit on iOS; Material on Android; GTK and Qt on Linux; WinUI and legacy XAML on Windows; ArkUI on HarmonyOS; and the browser DOM target. The legacy Windows target is included for comparison.

Host-specific capture notes:

- macOS GTK: set `PANGOCAIRO_BACKEND=fc` for complete Japanese font fallback with this host’s Homebrew Pango build.
- Linux Qt on `skipbox.local`: use `QT_QPA_PLATFORM=xcb QT_QPA_PLATFORMTHEME=gtk3 QT_STYLE_OVERRIDE=Fusion` under an isolated Xvfb/D-Bus session. Capture light and dark in separate runs, with `GTK_THEME=Adwaita` and `GTK_THEME=Adwaita:dark` respectively; the host’s Qt platform plugin otherwise ignores the requested dark scheme. An isolated `XDG_CONFIG_HOME` with `gtk-3.0/settings.ini` selecting `gtk-icon-theme-name=breeze` supplies the expected add/remove glyphs.
- Linux GTK on `skipbox.local`: use `GDK_BACKEND=x11 GSK_RENDERER=cairo xvfb-run -a -s '-screen 0 2600x2200x24' dbus-run-session -- scripts/capture.sh linux-gtk`. This isolates the display and D-Bus session and avoids a stalled GPU render checkpoint on this host.
- HarmonyOS: use a separate portrait phone emulator and set `DAY_OHOS_TARGET=<hdc-address>` as well as `--ohos-device <hdc-address>` so device-level screenshots use the same emulator. The review used a 360 × 720 viewport on `skipbox.local`. Its minimal image lacked the Noto Sans CJK font referenced by its font configuration; the existing Linux Noto Sans CJK package was installed into this separate emulator’s `/system/fonts/` and the emulator restarted to enable Korean glyphs.
- Windows: run `scripts/capture.ps1 -Target windows-winui -SmallBuild` (or `windows-xaml`) in the logged-in desktop session. An SSH shell alone may not have an interactive display. When copying macOS sources, exclude AppleDouble `._*` metadata files. On the space-constrained review VM, debug symbols and incremental compilation were disabled, and each target’s generated Cargo cache was removed after its screenshots were saved.
- Web: set `DAY_WEB_DRIVER` to the framework's `node .../day-cli/resources/web/webdom-driver.mjs`, `DAY_WEB_DRIVER_PLAYWRIGHT` to the installed Playwright package directory, and `DAY_WEB_DRIVER_BROWSER=chromium`.

## Local review gallery

Collect each host's target folders into `build/day/screenshots/`, then run:

```sh
python3 scripts/build_review.py --output "$HOME/Desktop/Day-Travel-Review"
open -a Safari "$HOME/Desktop/Day-Travel-Review/index.html"
```

The gallery works offline and filters by platform, language, and appearance. Mobile captures appear in pairs. Approvals and notes persist in the browser; **Export decisions** saves a portable JSON file. An exported file can be supplied for a later editing session. The page does not upload decisions or publish screenshots.

`manifest.json` records dimensions, screenshot checksums, and app source hashes. Capture logs and `validation.json` accompany this review build. Refreshing the gallery preserves notes; changed screenshot checksums clear only the affected approval. Missing captures remain visible and cannot be approved. The builder rejects invalid or tiny PNGs and identical top/bottom pairs. It does not substitute an image from another target. Optional `build/capture-issues.json` maps target identifiers to diagnostic notes.

Review UI strings live in `scripts/review/strings.en.json`; the builder and generated JavaScript accessors supply all UI copy from that catalog.

## Check and edit

```sh
cargo fmt --all -- --check
cargo test --offline
cargo clippy --offline --all-targets -- -D warnings
../day/target/debug/day lint --allow store-missing-field
../day/target/debug/day launch -t macos-appkit --locale en --script dayscript/demo.yaml
```

The store-field waiver covers missing privacy-policy URLs for this unpublished sample. `demo.yaml` exercises traveler counts, packing switches, and language selection; capture scripts preserve the selected locale instead.

- `src/lib.rs`: screen layout, appearance, and interactions.
- `resource/locales/<locale>/app.ftl`: translated UI and accessibility labels.
- `resource/vectors/`: illustrations and glyphs, accessed through generated resource symbols.
- `dayscript/`: interaction checks and capture walkthroughs.
- `scripts/`: capture wrapper and portable review gallery.

This project and its screenshots remain local until reviewed. The website does not yet consume them.
