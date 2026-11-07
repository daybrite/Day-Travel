#!/usr/bin/env bash
# Capture one installed native target. Pass device/driver options after the target.
# Examples: scripts/capture.sh ios-uikit --ios-simulator <simulator-UDID>
#           scripts/capture.sh android-material --android-device emulator-5554
set -euo pipefail
cd "$(dirname "$0")/.."
target=${1:?Usage: scripts/capture.sh TARGET [day launch options]}
shift
script=dayscript/capture-desktop.yaml
case "$target" in ios-uikit|android-material|harmony-arkui) script=dayscript/capture-mobile.yaml;; esac
cli=${DAY_CLI:-day}
exec "$cli" launch -t "$target" --script "$script" \
  --locales en,fr,de,ja,zh-CN,ko,ar --themes light,dark "$@"
