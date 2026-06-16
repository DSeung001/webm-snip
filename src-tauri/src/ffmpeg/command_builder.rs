use std::{
    env,
    path::{Path, PathBuf},
    process::Command,
};

use crate::models::{AppError, AppResult, ExportMode};

pub fn ffmpeg_command() -> AppResult<Command> {
    command("WEBM_SNIP_FFMPEG", "ffmpeg").ok_or_else(AppError::ffmpeg_not_found)
}

pub fn ffprobe_command() -> AppResult<Command> {
    command("WEBM_SNIP_FFPROBE", "ffprobe").ok_or_else(AppError::ffprobe_not_found)
}

pub fn build_segment_args(
    input: &Path,
    output: &Path,
    start: f64,
    duration: f64,
    mode: ExportMode,
) -> Vec<String> {
    let mut args = vec![
        "-hide_banner".to_string(),
        "-v".to_string(),
        "error".to_string(),
    ];

    match mode {
        ExportMode::Fast => {
            args.push("-ss".to_string());
            args.push(format_seconds(start));
            args.push("-i".to_string());
            args.push(input.to_string_lossy().to_string());
        }
        ExportMode::Accurate => {
            args.push("-i".to_string());
            args.push(input.to_string_lossy().to_string());
            args.push("-ss".to_string());
            args.push(format_seconds(start));
        }
    }

    args.extend([
        "-t".to_string(),
        format_seconds(duration),
        "-map".to_string(),
        "0".to_string(),
    ]);

    match mode {
        ExportMode::Fast => {
            args.push("-c".to_string());
            args.push("copy".to_string());
        }
        ExportMode::Accurate => {
            args.push("-c:v".to_string());
            args.push("libvpx-vp9".to_string());
            args.push("-c:a".to_string());
            args.push("libopus".to_string());
        }
    }

    args.push("-n".to_string());
    args.push(output.to_string_lossy().to_string());
    args
}

pub fn format_seconds(value: f64) -> String {
    format!("{:.3}", value.max(0.0))
}

fn command(env_var: &str, base_name: &str) -> Option<Command> {
    if let Ok(path) = env::var(env_var) {
        let path = PathBuf::from(path);
        if path.exists() {
            return Some(Command::new(path));
        }
    }

    for candidate in sidecar_candidates(base_name) {
        if candidate.exists() {
            return Some(Command::new(candidate));
        }
    }

    Some(Command::new(base_name))
}

fn sidecar_candidates(base_name: &str) -> Vec<PathBuf> {
    let binary_name = target_binary_name(base_name);
    let mut candidates = Vec::new();

    if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let binaries_dir = PathBuf::from(manifest_dir).join("binaries");
        candidates.push(binaries_dir.join(&binary_name));
        candidates.push(binaries_dir.join(base_name));
    }

    if let Ok(exe) = env::current_exe() {
        if let Some(exe_dir) = exe.parent() {
            candidates.push(exe_dir.join(&binary_name));
            candidates.push(exe_dir.join(base_name));
            candidates.push(exe_dir.join("../Resources").join(&binary_name));
            candidates.push(exe_dir.join("../Resources").join(base_name));
            candidates.push(exe_dir.join("resources").join(&binary_name));
            candidates.push(exe_dir.join("resources").join(base_name));
        }
    }

    candidates
}

fn target_binary_name(base_name: &str) -> String {
    let target = if cfg!(all(target_os = "macos", target_arch = "aarch64")) {
        "aarch64-apple-darwin"
    } else if cfg!(all(target_os = "macos", target_arch = "x86_64")) {
        "x86_64-apple-darwin"
    } else if cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        "x86_64-pc-windows-msvc"
    } else if cfg!(all(target_os = "linux", target_arch = "x86_64")) {
        "x86_64-unknown-linux-gnu"
    } else {
        "unknown"
    };

    if cfg!(target_os = "windows") {
        format!("{base_name}-{target}.exe")
    } else {
        format!("{base_name}-{target}")
    }
}
