use serde::Serialize;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenFileResult {
    pub path: String,
    pub name: String,
    pub size_bytes: u64,
}
