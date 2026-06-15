use serde::Serialize;

use super::ExportMode;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ExportResult {
    pub output_path: String,
    pub duration: f64,
    pub size_bytes: u64,
    pub mode: ExportMode,
}
