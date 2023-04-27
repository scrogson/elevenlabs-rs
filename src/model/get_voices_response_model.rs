use super::VoiceResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetVoicesResponseModel {
    pub voices: Vec<VoiceResponseModel>,
}

impl std::fmt::Display for GetVoicesResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
