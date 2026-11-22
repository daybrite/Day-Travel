# TravelBookings city photos

`Cities.json` is an unchanged copy of `skiptools/skipapp-bookings/Sources/TravelBookings/Resources/Cities.json`. Its 28 cities and their order drive the app's city navigation and native screenshot flows. All original metadata fields are preserved in this source snapshot; the current travel-planning UI uses the city names, photos, country names, and map coordinates. The imported taglines are preserved only as source data; reviewed, localized replacements and citations are in `city-captions.json` and [CAPTIONS.md](CAPTIONS.md).

`city-names.json` supplies city-name translations for the app's seven locales. `downloads.json` records each original photo URL, bundled file, byte size and SHA-256 hash. Photos were downloaded from the exact `imageURL` values, without changing the query parameters or substituting images. Their combined size is 3,630,195 bytes. They are JPEG resources under `resource/images/photo_*.jpg`, referenced through generated `res::images` constants and available offline.

From Day-Travel:

```sh
python3 scripts/download_city_photos.py
python3 scripts/generate_destinations.py
cargo fmt --all
day build -t macos-appkit
day launch -t macos-appkit --skip-build
```

The destination generator requires every photo and every city-name translation to exist. The capture scripts visit all 28 cities in source order. The native controls walkthrough exercises Tokyo’s date and planning controls, plus trip saving and language switching.
