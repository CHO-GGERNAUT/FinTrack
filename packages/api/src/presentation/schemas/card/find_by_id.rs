use crate::application::dto::DeleteCardOutput;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCardRequest {
    pub account_id: Uuid,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteCardResponse {
    pub account_id: Uuid,
}

impl From<DeleteCardOutput> for DeleteCardResponse {
    fn from(output: DeleteCardOutput) -> Self {
        Self {
            account_id: output.account_id,
        }
    }
}
