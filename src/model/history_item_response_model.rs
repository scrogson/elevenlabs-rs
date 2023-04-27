use super::FeedbackResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HistoryItemResponseModel {
    pub character_count_change_from: i64,
    pub character_count_change_to: i64,
    pub content_type: String,
    pub date_unix: i64,
    pub feedback: FeedbackResponseModel,
    pub history_item_id: String,
    pub request_id: String,
    pub settings: serde_json::Value,
    pub state: String,
    pub text: String,
    pub voice_id: String,
    pub voice_name: String,
}

impl std::fmt::Display for HistoryItemResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
