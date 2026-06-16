# FFmpeg Sidecars

Release builds bundle FFmpeg and FFprobe through Tauri `externalBin`.

The prepare script writes platform-specific names here before packaging:

- `ffmpeg-x86_64-pc-windows-msvc.exe`
- `ffprobe-x86_64-pc-windows-msvc.exe`
- `ffmpeg-aarch64-apple-darwin`
- `ffprobe-aarch64-apple-darwin`
- `ffmpeg-x86_64-apple-darwin`
- `ffprobe-x86_64-apple-darwin`
- `ffmpeg-x86_64-unknown-linux-gnu`
- `ffprobe-x86_64-unknown-linux-gnu`

For local development, set `WEBM_SNIP_FFMPEG` and `WEBM_SNIP_FFPROBE`, or install
`ffmpeg` and `ffprobe` on `PATH`.
