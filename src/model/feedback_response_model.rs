use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct FeedbackResponseModel {
    pub audio_quality: bool,
    pub emotions: bool,
    pub feedback: String,
    pub glitches: bool,
    pub inaccurate_clone: bool,
    pub other: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub review_status: Option<String>,
    pub thumbs_up: bool,
}

impl std::fmt::Display for FeedbackResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
