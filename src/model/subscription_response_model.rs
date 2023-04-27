use super::TtsModelResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct SubscriptionResponseModel {
    pub allowed_to_extend_character_limit: bool,
    pub available_models: Vec<TtsModelResponseModel>,
    pub can_extend_character_limit: bool,
    pub can_extend_voice_limit: bool,
    pub can_use_delayed_payment_methods: bool,
    pub can_use_instant_voice_cloning: bool,
    pub can_use_professional_voice_cloning: bool,
    pub character_count: i64,
    pub character_limit: i64,
    pub currency: String,
    pub next_character_count_reset_unix: i64,
    pub professional_voice_limit: i64,
    pub status: String,
    pub tier: String,
    pub voice_limit: i64,
}

impl std::fmt::Display for SubscriptionResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
