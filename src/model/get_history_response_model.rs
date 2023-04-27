use super::HistoryItemResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct GetHistoryResponseModel {
    pub history: Vec<HistoryItemResponseModel>,
}

impl std::fmt::Display for GetHistoryResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
