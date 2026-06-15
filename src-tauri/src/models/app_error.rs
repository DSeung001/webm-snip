use serde::Serialize;

pub type AppResult<T> = Result<T, AppError>;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppError {
    pub code: &'static str,
    pub message: &'static str,
    pub detail: Option<String>,
}

impl AppError {
    pub fn new(code: &'static str, message: &'static str) -> Self {
        Self {
            code,
            message,
            detail: None,
        }
    }

    pub fn with_detail(code: &'static str, message: &'static str, detail: impl ToString) -> Self {
        Self {
            code,
            message,
            detail: Some(detail.to_string()),
        }
    }

    pub fn invalid_file_type() -> Self {
        Self::new("INVALID_FILE_TYPE", "Only WebM files can be opened.")
    }

    pub fn file_not_found() -> Self {
        Self::new("FILE_NOT_FOUND", "File not found.")
    }

    pub fn invalid_cut_range() -> Self {
        Self::new("INVALID_CUT_RANGE", "Check the selected range.")
    }

    pub fn output_path_invalid() -> Self {
        Self::new("OUTPUT_PATH_INVALID", "The output path is not valid.")
    }

    pub fn ffmpeg_not_found() -> Self {
        Self::new(
            "FFMPEG_NOT_FOUND",
            "The video processing module was not found. Reinstall the app.",
        )
    }

    pub fn ffprobe_not_found() -> Self {
        Self::new(
            "FFPROBE_NOT_FOUND",
            "The video analysis module was not found. Reinstall the app.",
        )
    }

    pub fn ffmpeg_failed(detail: impl ToString) -> Self {
        Self::with_detail("FFMPEG_FAILED", "Export failed. Try Accurate mode.", detail)
    }

    pub fn ffprobe_failed(detail: impl ToString) -> Self {
        Self::with_detail(
            "FFPROBE_FAILED",
            "Could not read video info. Try another WebM file.",
            detail,
        )
    }

    pub fn permission_denied(detail: impl ToString) -> Self {
        Self::with_detail(
            "PERMISSION_DENIED",
            "Permission denied for this file location.",
            detail,
        )
    }
}
