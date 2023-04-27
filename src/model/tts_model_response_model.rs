use super::LanguageResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TtsModelResponseModel {
    pub display_name: String,
    pub model_id: String,
    pub supported_language: Vec<LanguageResponseModel>,
}

impl std::fmt::Display for TtsModelResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
