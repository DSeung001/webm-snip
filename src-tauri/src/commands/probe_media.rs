use std::{fs, io::ErrorKind, path::PathBuf};

use serde_json::Value;

use crate::{
    ffmpeg::{command_builder::ffprobe_command, validator::ensure_input_file},
    models::{AppError, AppResult, MediaInfo},
};

#[tauri::command]
pub fn probe_media(path: String) -> AppResult<MediaInfo> {
    let input = PathBuf::from(path);
    ensure_input_file(&input)?;

    let mut command = ffprobe_command()?;
    let output = command
        .args([
            "-v",
            "error",
            "-print_format",
            "json",
            "-show_format",
            "-show_streams",
        ])
        .arg(&input)
        .output()
        .map_err(|error| {
            if error.kind() == ErrorKind::NotFound {
                AppError::ffprobe_not_found()
            } else {
                AppError::ffprobe_failed(error)
            }
        })?;

    if !output.status.success() {
        return Err(AppError::ffprobe_failed(String::from_utf8_lossy(
            &output.stderr,
        )));
    }

    let json: Value = serde_json::from_slice(&output.stdout).map_err(AppError::ffprobe_failed)?;
    media_info_from_probe(&input, json)
}

pub fn media_info_from_probe(input: &PathBuf, json: Value) -> AppResult<MediaInfo> {
    let metadata = fs::metadata(input).map_err(AppError::permission_denied)?;
    let streams = json
        .get("streams")
        .and_then(Value::as_array)
        .ok_or_else(|| AppError::ffprobe_failed("missing streams"))?;

    let format_duration = json
        .get("format")
        .and_then(|format| format.get("duration"))
        .and_then(Value::as_str)
        .and_then(|value| value.parse::<f64>().ok());

    let mut video_codec = None;
    let mut audio_codec = None;
    let mut width = None;
    let mut height = None;
    let mut fps = None;
    let mut stream_duration = None;
    let mut video_start_time = 0.0_f64;

    for stream in streams {
        if stream_duration.is_none() {
            stream_duration = stream
                .get("duration")
                .and_then(Value::as_str)
                .and_then(|value| value.parse::<f64>().ok());
        }

        match stream.get("codec_type").and_then(Value::as_str) {
            Some("video") if video_codec.is_none() => {
                video_start_time = stream
                    .get("start_time")
                    .and_then(Value::as_str)
                    .and_then(|value| value.parse::<f64>().ok())
                    .unwrap_or(0.0);
                video_codec = stream
                    .get("codec_name")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);
                width = stream
                    .get("width")
                    .and_then(Value::as_u64)
                    .and_then(|value| u32::try_from(value).ok());
                height = stream
                    .get("height")
                    .and_then(Value::as_u64)
                    .and_then(|value| u32::try_from(value).ok());
                fps = stream
                    .get("avg_frame_rate")
                    .and_then(Value::as_str)
                    .and_then(parse_fraction);
            }
            Some("audio") if audio_codec.is_none() => {
                audio_codec = stream
                    .get("codec_name")
                    .and_then(Value::as_str)
                    .map(ToString::to_string);
            }
            _ => {}
        }
    }

    let duration = format_duration
        .or(stream_duration)
        .ok_or_else(|| AppError::ffprobe_failed("missing duration"))?;

    Ok(MediaInfo {
        path: input.to_string_lossy().to_string(),
        filename: input
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("video.webm")
            .to_string(),
        duration,
        video_start_time,
        size_bytes: metadata.len(),
        width,
        height,
        video_codec,
        audio_codec,
        fps,
    })
}

fn parse_fraction(value: &str) -> Option<f64> {
    let (left, right) = value.split_once('/')?;
    let numerator = left.parse::<f64>().ok()?;
    let denominator = right.parse::<f64>().ok()?;
    if denominator == 0.0 {
        None
    } else {
        Some(numerator / denominator)
    }
}
