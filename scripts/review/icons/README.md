# Preview control icons

Platform SVGs are unchanged copies of `day/website/src/icons/`. See [SOURCES.md](SOURCES.md) for their sources, licenses, and attribution. Copy updated originals from that directory when the site icons change. The widget shows GTK and Qt on Linux; the macOS option uses AppKit.

`sun.svg` and `moon.svg` are original geometric theme indicators. The preview builder embeds the vectors and generates symbolic `res.vectors` accessors, so the offline page needs no sibling checkout or network access.

Platform colors follow the light and dark palettes in `day/website/src/styles/global.css`. Locale controls display each language's native name from the capture metadata.
