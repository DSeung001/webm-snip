use std::{io::ErrorKind, path::PathBuf};

use crate::{
    commands::{file_manager::resolve_output_path, probe_media::probe_media},
    ffmpeg::{
        command_builder::{build_cut_args, ffmpeg_command},
        validator::{ensure_cut_range, ensure_input_file, ensure_nonempty_output},
    },
    models::{AppError, AppResult, CutRequest, ExportResult},
};

#[tauri::command]
pub fn cut_webm(request: CutRequest) -> AppResult<ExportResult> {
    let input = PathBuf::from(&request.input_path);
    ensure_input_file(&input)?;

    let media = probe_media(request.input_path.clone())?;
    ensure_cut_range(request.start_time, request.end_time, Some(media.duration))?;

    let output = resolve_output_path(&input, request.output_path)?;
    let duration = request.end_time - request.start_time;
    let args = build_cut_args(&input, &output, request.start_time, duration, request.mode);

    let mut command = ffmpeg_command()?;
    let output_status = command.args(args).output().map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            AppError::ffmpeg_not_found()
        } else {
            AppError::ffmpeg_failed(error)
        }
    })?;

    if !output_status.status.success() {
        return Err(AppError::ffmpeg_failed(String::from_utf8_lossy(
            &output_status.stderr,
        )));
    }

    let size_bytes = ensure_nonempty_output(&output)?;
    let result_media = probe_media(output.to_string_lossy().to_string())?;

    Ok(ExportResult {
        output_path: output.to_string_lossy().to_string(),
        duration: result_media.duration,
        size_bytes,
        mode: request.mode,
    })
}
