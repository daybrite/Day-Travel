# Sourced by dayapp's script-setup hook after the selected device is booted.
# Keep environment changes in the runner process; do not edit the checkout.
check_capture_coverage() {
    local result=$?
    if [ "$result" -eq 0 ]; then
        python3 scripts/ci/check-screenshots.py \
            --root "${DAY_BUILD_DIR:-build/day}/screenshots" \
            --target "$DAY_SCRIPT_TARGET" || result=$?
    fi
    exit "$result"
}
trap check_capture_coverage EXIT

case "${DAY_SCRIPT_TARGET:?dayapp must name the capture target}" in
  macos-gtk)
    # Homebrew Pango's CoreText font map missed Japanese glyphs. Fontconfig finds
    # the installed CJK faces without changing the app's semantic text styles.
    export PANGOCAIRO_BACKEND=fc
    ;;
  harmony-arkui)
    # Host fonts do not reach a QEMU guest. The minimal image's configuration
    # references Noto Sans CJK, but does not contain the font needed for Korean.
    python3 scripts/ci/prepare-harmony-fonts.py
    ;;
esac
