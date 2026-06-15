use std::{
    fs::{self, File},
    io::{ErrorKind, Write},
    path::{Path, PathBuf},
    time::{SystemTime, UNIX_EPOCH},
};

use crate::{
    commands::{file_manager::resolve_output_path, probe_media::probe_media},
    ffmpeg::{
        command_builder::{build_segment_args, ffmpeg_command},
        validator::{ensure_cut_range, ensure_input_file, ensure_nonempty_output},
    },
    models::{AppError, AppResult, ExportMode, ExportResult, TimelineExportRequest},
};

#[tauri::command]
pub fn export_timeline(request: TimelineExportRequest) -> AppResult<ExportResult> {
    if request.input_paths.is_empty() || request.segments.is_empty() {
        return Err(AppError::invalid_cut_range());
    }

    let inputs = request
        .input_paths
        .iter()
        .map(PathBuf::from)
        .collect::<Vec<_>>();

    for input in &inputs {
        ensure_input_file(input)?;
    }

    let first_input = inputs.first().ok_or_else(AppError::output_path_invalid)?;
    let output = resolve_output_path(first_input, request.output_path)?;
    let temp_dir = create_temp_dir()?;
    let mut segment_files = Vec::new();

    let export_result = (|| {
        for (index, segment) in request.segments.iter().enumerate() {
            let Some(input) = inputs.get(segment.input_index) else {
                return Err(AppError::invalid_cut_range());
            };

            let media = probe_media(input.to_string_lossy().to_string())?;
            let media_end = media.video_start_time + media.duration;
            ensure_cut_range(segment.start_time, segment.end_time, Some(media_end))?;

            let duration = segment.end_time - segment.start_time;
            let segment_output = temp_dir.join(format!("segment_{index:04}.webm"));

            export_segment(
                input,
                &segment_output,
                segment.start_time,
                duration,
                request.mode,
            )?;
            ensure_nonempty_output(&segment_output)?;
            segment_files.push(segment_output);
        }

        let concat_list = temp_dir.join("concat.txt");
        write_concat_list(&concat_list, &segment_files)?;
        concat_segments(&concat_list, &output)?;

        let size_bytes = ensure_nonempty_output(&output)?;
        let result_media = probe_media(output.to_string_lossy().to_string())?;

        Ok(ExportResult {
            output_path: output.to_string_lossy().to_string(),
            duration: result_media.duration,
            size_bytes,
            mode: request.mode,
        })
    })();

    let _ = fs::remove_dir_all(&temp_dir);
    export_result
}

fn export_segment(
    input: &Path,
    output: &Path,
    start: f64,
    duration: f64,
    mode: ExportMode,
) -> AppResult<()> {
    let mut args = build_segment_args(input, output, start, duration, mode);
    if let Some(position) = args.iter().position(|arg| arg == "-n") {
        args[position] = "-y".to_string();
    }

    run_ffmpeg(args)
}

fn concat_segments(list_path: &Path, output: &Path) -> AppResult<()> {
    run_ffmpeg(vec![
        "-hide_banner".to_string(),
        "-v".to_string(),
        "error".to_string(),
        "-f".to_string(),
        "concat".to_string(),
        "-safe".to_string(),
        "0".to_string(),
        "-i".to_string(),
        list_path.to_string_lossy().to_string(),
        "-c".to_string(),
        "copy".to_string(),
        "-n".to_string(),
        output.to_string_lossy().to_string(),
    ])
}

fn run_ffmpeg(args: Vec<String>) -> AppResult<()> {
    let mut command = ffmpeg_command()?;
    let output_status = command.args(args).output().map_err(|error| {
        if error.kind() == ErrorKind::NotFound {
            AppError::ffmpeg_not_found()
        } else {
            AppError::ffmpeg_failed(error)
        }
    })?;

    if output_status.status.success() {
        Ok(())
    } else {
        Err(AppError::ffmpeg_failed(String::from_utf8_lossy(
            &output_status.stderr,
        )))
    }
}

fn write_concat_list(path: &Path, segment_files: &[PathBuf]) -> AppResult<()> {
    let mut file = File::create(path).map_err(AppError::permission_denied)?;
    for segment_file in segment_files {
        writeln!(file, "file '{}'", escape_concat_path(segment_file))
            .map_err(AppError::permission_denied)?;
    }
    Ok(())
}

fn escape_concat_path(path: &Path) -> String {
    path.to_string_lossy().replace('\'', "'\\''")
}

fn create_temp_dir() -> AppResult<PathBuf> {
    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(AppError::permission_denied)?
        .as_millis();
    let dir = std::env::temp_dir().join(format!("webm-snip-{}-{timestamp}", std::process::id()));
    fs::create_dir_all(&dir).map_err(AppError::permission_denied)?;
    Ok(dir)
}
