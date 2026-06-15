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
        Self::new("INVALID_FILE_TYPE", "WebM 파일만 열 수 있습니다.")
    }

    pub fn file_not_found() -> Self {
        Self::new("FILE_NOT_FOUND", "파일을 찾을 수 없습니다.")
    }

    pub fn invalid_cut_range() -> Self {
        Self::new("INVALID_CUT_RANGE", "시작점과 끝점을 다시 확인해 주세요.")
    }

    pub fn output_path_invalid() -> Self {
        Self::new("OUTPUT_PATH_INVALID", "저장 위치가 올바르지 않습니다.")
    }

    pub fn ffmpeg_not_found() -> Self {
        Self::new(
            "FFMPEG_NOT_FOUND",
            "영상 처리 모듈을 찾을 수 없습니다. 앱을 다시 설치해 주세요.",
        )
    }

    pub fn ffprobe_not_found() -> Self {
        Self::new(
            "FFPROBE_NOT_FOUND",
            "영상 분석 모듈을 찾을 수 없습니다. 앱을 다시 설치해 주세요.",
        )
    }

    pub fn ffmpeg_failed(detail: impl ToString) -> Self {
        Self::with_detail(
            "FFMPEG_FAILED",
            "영상 자르기에 실패했습니다. 정확하게 자르기로 다시 시도해 주세요.",
            detail,
        )
    }

    pub fn ffprobe_failed(detail: impl ToString) -> Self {
        Self::with_detail(
            "FFPROBE_FAILED",
            "영상 정보를 읽지 못했습니다. 다른 WebM 파일로 다시 시도해 주세요.",
            detail,
        )
    }

    pub fn permission_denied(detail: impl ToString) -> Self {
        Self::with_detail(
            "PERMISSION_DENIED",
            "이 위치에 파일을 저장할 권한이 없습니다.",
            detail,
        )
    }
}
