use std::{fs, path::Path};

use crate::models::{AppError, AppResult};

pub fn ensure_webm_path(path: &Path) -> AppResult<()> {
    if path
        .extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| extension.eq_ignore_ascii_case("webm"))
        .unwrap_or(false)
    {
        Ok(())
    } else {
        Err(AppError::invalid_file_type())
    }
}

pub fn ensure_input_file(path: &Path) -> AppResult<()> {
    ensure_webm_path(path)?;
    if path.exists() {
        Ok(())
    } else {
        Err(AppError::file_not_found())
    }
}

pub fn ensure_output_path(input: &Path, output: &Path) -> AppResult<()> {
    ensure_webm_path(output)?;

    if same_path(input, output) {
        return Err(AppError::output_path_invalid());
    }

    if output.exists() {
        return Err(AppError::with_detail(
            "OUTPUT_ALREADY_EXISTS",
            "A file with this name already exists.",
            output.display(),
        ));
    }

    match output.parent() {
        Some(parent) if parent.exists() => Ok(()),
        _ => Err(AppError::output_path_invalid()),
    }
}

pub fn ensure_cut_range(start: f64, end: f64, duration: Option<f64>) -> AppResult<()> {
    if !start.is_finite() || !end.is_finite() || start < 0.0 || end <= start || end - start < 0.3 {
        return Err(AppError::invalid_cut_range());
    }

    if let Some(duration) = duration {
        if end > duration + 0.01 {
            return Err(AppError::invalid_cut_range());
        }
    }

    Ok(())
}

pub fn ensure_nonempty_output(path: &Path) -> AppResult<u64> {
    let metadata = fs::metadata(path).map_err(|error| AppError::permission_denied(error))?;
    if metadata.len() == 0 {
        Err(AppError::ffmpeg_failed("output file is empty"))
    } else {
        Ok(metadata.len())
    }
}

fn same_path(left: &Path, right: &Path) -> bool {
    match (left.canonicalize(), right.canonicalize()) {
        (Ok(left), Ok(right)) => left == right,
        _ => left == right,
    }
}
