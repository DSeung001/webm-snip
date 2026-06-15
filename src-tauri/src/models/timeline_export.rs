use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum ExportMode {
    Fast,
    Accurate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineSegment {
    pub input_index: usize,
    pub start_time: f64,
    pub end_time: f64,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TimelineExportRequest {
    pub input_paths: Vec<String>,
    pub output_path: Option<String>,
    pub segments: Vec<TimelineSegment>,
    pub mode: ExportMode,
}
