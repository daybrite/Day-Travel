#!/usr/bin/env bash
# Preserve the SVG viewport at 2x for Windows' gradient fallback. Requires resvg.
set -euo pipefail
cd "$(dirname "$0")/.."
mkdir -p resource/images
for name in valletta valletta_dark valletta_detail valletta_detail_dark; do
    width=2400
    case "$name" in *detail*) width=1440;; esac
    resvg --width "$width" --height 560 "resource/vectors/$name.svg" "resource/images/${name}_windows.png"
done
