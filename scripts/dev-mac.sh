#!/usr/bin/env sh
set -eu

if [ "$(uname -s)" != "Darwin" ]; then
  echo "This script is for macOS development. Use 'pnpm tauri:dev' on other hosts." >&2
  exit 1
fi

if ! command -v pnpm >/dev/null 2>&1; then
  echo "pnpm is required. Install it first, then run this script again." >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "Rust/Cargo is required. Install Rust with rustup, then run this script again." >&2
  exit 1
fi

if [ -z "${WEBM_SNIP_FFMPEG:-}" ]; then
  if command -v ffmpeg >/dev/null 2>&1; then
    WEBM_SNIP_FFMPEG="$(command -v ffmpeg)"
    export WEBM_SNIP_FFMPEG
  else
    echo "ffmpeg is required. Install it with 'brew install ffmpeg' or set WEBM_SNIP_FFMPEG." >&2
    exit 1
  fi
fi

if [ -z "${WEBM_SNIP_FFPROBE:-}" ]; then
  if command -v ffprobe >/dev/null 2>&1; then
    WEBM_SNIP_FFPROBE="$(command -v ffprobe)"
    export WEBM_SNIP_FFPROBE
  else
    echo "ffprobe is required. Install it with 'brew install ffmpeg' or set WEBM_SNIP_FFPROBE." >&2
    exit 1
  fi
fi

case "$(uname -m)" in
  arm64) tauri_cli_package="@tauri-apps+cli-darwin-arm64" ;;
  x86_64) tauri_cli_package="@tauri-apps+cli-darwin-x64" ;;
  *) tauri_cli_package="" ;;
esac

if [ -n "$tauri_cli_package" ] && [ -d "node_modules/.pnpm" ]; then
  if ! find node_modules/.pnpm -maxdepth 1 -type d -name "${tauri_cli_package}@*" | grep -q .; then
    echo "macOS Tauri CLI native binding is missing." >&2
    echo "Run 'pnpm install --force' on the Mac host, then try again." >&2
    echo "Docker node_modules is isolated by compose.yaml, so this should be a one-time repair." >&2
    exit 1
  fi
fi

echo "Using ffmpeg:  $WEBM_SNIP_FFMPEG"
echo "Using ffprobe: $WEBM_SNIP_FFPROBE"

if command -v lsof >/dev/null 2>&1 && lsof -nP -iTCP:1420 -sTCP:LISTEN >/dev/null 2>&1; then
  echo "Port 1420 is already in use. Reusing the existing frontend dev server."
  pnpm exec tauri dev --config '{"build":{"beforeDevCommand":"echo Reusing existing Vite dev server on http://localhost:1420"}}'
else
  pnpm exec tauri dev
fi
