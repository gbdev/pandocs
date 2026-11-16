#!/usr/bin/env bash
# Generates `pandocs.pdf` from the single-page version, using a headless Chrome/Chromium.
# Run this after `mdbook build`. Usage: tools/gen_pdf.sh [build dir]
# The browser binary can be overridden with the `CHROME` environment variable.
set -euo pipefail

BUILD_DIR="$(realpath "${1:-docs/pandocs}")"
INPUT="$BUILD_DIR/single.html"
OUTPUT="$BUILD_DIR/pandocs.pdf"

if [[ ! -f "$INPUT" ]]; then
	echo "error: $INPUT not found; run \`mdbook build\` first" >&2
	exit 1
fi

if [[ -z "${CHROME:-}" ]]; then
	for candidate in google-chrome google-chrome-stable chromium chromium-browser; do
		if command -v "$candidate" >/dev/null; then
			CHROME="$candidate"
			break
		fi
	done
fi
if [[ -z "${CHROME:-}" ]]; then
	echo "error: no Chrome/Chromium found; set the CHROME environment variable" >&2
	exit 1
fi

# The page is our own build output, so the sandbox can be disabled
# (it fails to start on some CI runners).
"$CHROME" --headless=new --no-sandbox --disable-gpu \
	--no-pdf-header-footer --generate-pdf-document-outline \
	--print-to-pdf="$OUTPUT" "file://$INPUT"

[[ -s "$OUTPUT" ]] || { echo "error: PDF was not generated" >&2; exit 1; }
echo "Generated $OUTPUT"
