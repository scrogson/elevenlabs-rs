use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct VoiceSettingsResponseModel {
    pub similarity_boost: f64,
    pub stability: f64,
}

impl std::fmt::Display for VoiceSettingsResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
