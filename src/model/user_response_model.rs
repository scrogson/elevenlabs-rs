use super::SubscriptionResponseModel;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserResponseModel {
    pub is_new_user: bool,
    pub subscription: SubscriptionResponseModel,
    pub xi_api_key: String,
}

impl std::fmt::Display for UserResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
