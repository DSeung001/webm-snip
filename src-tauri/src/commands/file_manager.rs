use std::{
    fs,
    path::{Path, PathBuf},
    process::Command,
};

use crate::{
    ffmpeg::validator::{ensure_input_file, ensure_output_path, ensure_webm_path},
    models::{AppError, AppResult, OpenFileResult},
};

#[tauri::command]
pub fn open_webm_file() -> AppResult<Option<OpenFileResult>> {
    let Some(path) = rfd::FileDialog::new()
        .add_filter("WebM", &["webm"])
        .pick_file()
    else {
        return Ok(None);
    };

    ensure_input_file(&path)?;
    let metadata = fs::metadata(&path).map_err(AppError::permission_denied)?;

    Ok(Some(OpenFileResult {
        path: path.to_string_lossy().to_string(),
        name: path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or("video.webm")
            .to_string(),
        size_bytes: metadata.len(),
    }))
}

#[tauri::command]
pub fn generate_default_output_path(input_path: String) -> AppResult<String> {
    let input = PathBuf::from(input_path);
    ensure_input_file(&input)?;
    Ok(generate_next_output_path(&input)?
        .to_string_lossy()
        .to_string())
}

#[tauri::command]
pub fn choose_save_path(default_path: String) -> AppResult<Option<String>> {
    let default_path = PathBuf::from(default_path);
    ensure_webm_path(&default_path)?;

    let directory = default_path
        .parent()
        .map(Path::to_path_buf)
        .unwrap_or_else(|| dirs::home_dir().unwrap_or_else(|| PathBuf::from(".")));
    let file_name = default_path
        .file_name()
        .and_then(|name| name.to_str())
        .unwrap_or("clip.webm");

    let Some(path) = rfd::FileDialog::new()
        .set_directory(directory)
        .set_file_name(file_name)
        .add_filter("WebM", &["webm"])
        .save_file()
    else {
        return Ok(None);
    };

    ensure_webm_path(&path)?;
    Ok(Some(path.to_string_lossy().to_string()))
}

#[tauri::command]
pub fn open_file_path(path: String) -> AppResult<()> {
    open_path(Path::new(&path))
}

#[tauri::command]
pub fn open_folder_path(path: String) -> AppResult<()> {
    let path = Path::new(&path);
    let target = if path.is_file() {
        path.parent().unwrap_or(path)
    } else {
        path
    };
    open_path(target)
}

pub fn resolve_output_path(input: &Path, requested: Option<String>) -> AppResult<PathBuf> {
    let output = match requested {
        Some(path) if !path.trim().is_empty() => PathBuf::from(path),
        _ => generate_next_output_path(input)?,
    };

    ensure_output_path(input, &output)?;
    Ok(output)
}

fn generate_next_output_path(input: &Path) -> AppResult<PathBuf> {
    let parent = input.parent().ok_or_else(AppError::output_path_invalid)?;
    let stem = input
        .file_stem()
        .and_then(|stem| stem.to_str())
        .ok_or_else(AppError::output_path_invalid)?;

    for index in 1..1000 {
        let candidate = parent.join(format!("{stem}_cut_{index:03}.webm"));
        if !candidate.exists() {
            return Ok(candidate);
        }
    }

    Err(AppError::output_path_invalid())
}

fn open_path(path: &Path) -> AppResult<()> {
    if !path.exists() {
        return Err(AppError::file_not_found());
    }

    let status = if cfg!(target_os = "macos") {
        Command::new("open").arg(path).status()
    } else if cfg!(target_os = "windows") {
        Command::new("explorer").arg(path).status()
    } else {
        Command::new("xdg-open").arg(path).status()
    }
    .map_err(AppError::permission_denied)?;

    if status.success() {
        Ok(())
    } else {
        Err(AppError::permission_denied(format!(
            "open command exited with {status}"
        )))
    }
}
