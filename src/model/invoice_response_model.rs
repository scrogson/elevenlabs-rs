use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct InvoiceResponseModel {
    pub amount_due_cents: i64,
    pub next_payment_attempt_unix: i64,
}

impl std::fmt::Display for InvoiceResponseModel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}
