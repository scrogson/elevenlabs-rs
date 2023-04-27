use super::VerificationAttemptResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FineTuningResponseModel {
    pub fine_tuning_requested: bool,
    pub finetuning_state: String,
    pub is_allowed_to_fine_tune: bool,
    pub model_id: String,
    pub slice_ids: Vec<String>,
    pub verification_attempts: Vec<VerificationAttemptResponseModel>,
    pub verification_attempts_count: i64,
    pub verification_failures: Vec<String>,
}

impl std::fmt::Display for FineTuningResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
