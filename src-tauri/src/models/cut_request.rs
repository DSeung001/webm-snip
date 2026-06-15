use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
pub enum CutMode {
    Fast,
    Accurate,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CutRequest {
    pub input_path: String,
    pub output_path: Option<String>,
    pub start_time: f64,
    pub end_time: f64,
    pub mode: CutMode,
}
