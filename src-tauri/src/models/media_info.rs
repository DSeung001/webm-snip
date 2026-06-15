use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    pub path: String,
    pub filename: String,
    pub duration: f64,
    /// First video packet timestamp from ffprobe (seconds). HTML5 currentTime is relative to this.
    pub video_start_time: f64,
    pub size_bytes: u64,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub video_codec: Option<String>,
    pub audio_codec: Option<String>,
    pub fps: Option<f64>,
}
