# WebM Snip

WebM Snip is a lightweight local desktop cutter for WebM ㅊ flow:

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
pnpm tauri:dev
```

For local development without bundled sidecars, install FFmpeg on `PATH` or set:

```bash
export WEBM_SNIP_FFMPEG=/path/to/ffmpeg
export WEBM_SNIP_FFPROBE=/path/to/ffprobe
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

## FFmpeg Sidecars

Put release binaries in `src-tauri/binaries` using the names documented in
`src-tauri/binaries/README.md`.

When release binaries are present, add Tauri `bundle.externalBin` entries for
`binaries/ffmpeg` and `binaries/ffprobe` before packaging. The entries are not
enabled in this repository yet because Tauri fails the build if the binaries are
missing.

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