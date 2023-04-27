use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SampleResponseModel {
    pub file_name: String,
    pub hash: String,
    pub mime_type: String,
    pub sample_id: String,
    pub size_bytes: i64,
}

impl std::fmt::Display for SampleResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
