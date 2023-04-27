use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct RecordingResponseModel {
    pub mime_type: String,
    pub recording_id: String,
    pub size_bytes: i64,
    pub transcription: String,
    pub upload_date_unix: i64,
}

impl std::fmt::Display for RecordingResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
