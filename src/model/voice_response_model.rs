use super::{FineTuningResponseModel, SampleResponseModel, VoiceSettingsResponseModel};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceResponseModel {
    pub available_for_tiers: Vec<String>,
    pub category: String,
    pub description: String,
    pub fine_tuning: FineTuningResponseModel,
    pub labels: serde_json::Value,
    pub name: String,
    pub preview_url: String,
    pub samples: Vec<SampleResponseModel>,
    pub settings: VoiceSettingsResponseModel,
    pub voice_id: String,
}

impl std::fmt::Display for VoiceResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
