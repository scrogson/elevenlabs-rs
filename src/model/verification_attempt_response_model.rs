use super::RecordingResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VerificationAttemptResponseModel {
    pub accepted: bool,
    pub date_unix: i64,
    pub levenshtein_distance: f64,
    pub recording: RecordingResponseModel,
    pub similarity: f64,
    pub text: String,
}

impl std::fmt::Display for VerificationAttemptResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
