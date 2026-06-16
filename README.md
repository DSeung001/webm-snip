# WebM Snip

WebM Snip is a lightweight local desktop cutter for WebM screen recordings.

Core flow:

```txt
Open -> Mark -> Cut
```

## Stack

- Tauri v2
- Svelte + TypeScript + Vite
- Rust commands
- FFmpeg / FFprobe sidecar-ready media engine
- pnpm
- Docker-isolated development environment

## Local Development

Install dependencies on the host:

```bash
pnpm install
pnpm tauri:dev:mac
```

On macOS, the dedicated script checks Rust, pnpm, FFmpeg, FFprobe, and the
macOS Tauri CLI native binding before starting desktop development mode.

If FFmpeg is not installed:

```bash
brew install ffmpeg
```

For local development without bundled sidecars, keep FFmpeg on `PATH` or set:

```bash
export WEBM_SNIP_FFMPEG=/path/to/ffmpeg
export WEBM_SNIP_FFPROBE=/path/to/ffprobe
```

If `tauri dev` reports a missing native binding after using Docker, repair the
host install once:

```bash
pnpm install --force
pnpm tauri:dev:mac
```

## Docker Development

The Docker setup keeps Node, pnpm, Rust, Tauri Linux build dependencies, FFmpeg,
and FFprobe inside a container:

```bash
docker compose up --build
```

This starts the Vite dev server at `http://localhost:1420`.

Tauri desktop windows are best run on the host for macOS/Windows packaging. The
container is intended for isolated web UI development, dependency checks, and
Linux-oriented build validation.

Docker uses its own `node_modules` volume so Linux-only optional dependencies do
not overwrite the macOS host install.

## FFmpeg Sidecars

Release builds bundle FFmpeg and FFprobe automatically.

Before `pnpm tauri:build`, the prepare script copies local binaries into
`src-tauri/binaries` using Tauri sidecar names such as
`ffmpeg-aarch64-apple-darwin` and `ffprobe-aarch64-apple-darwin`.

Install FFmpeg on the build machine first:

```bash
brew install ffmpeg
pnpm tauri:build
```

You can also point to custom binaries:

```bash
export WEBM_SNIP_FFMPEG=/path/to/ffmpeg
export WEBM_SNIP_FFPROBE=/path/to/ffprobe
pnpm tauri:build
```

For local development without bundled sidecars, keep FFmpeg on `PATH` or set the
environment variables above and run `pnpm tauri:dev:mac`.

## v0.1 Scope

- Open one `.webm` file
- Preview video
- Mark one start point and one end point
- Validate the selected range
- Save a new `.webm` next to the source file
- Open the result file or containing folder
- Retry with accurate cutting after a fast cut

Out of scope for v0.1: MP4/MOV/MKV support, multiple ranges, filters, captions,
BGM, project files, cloud upload, and accounts.
