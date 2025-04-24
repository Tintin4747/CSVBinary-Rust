use serde::Serialize;
use uuid::Uuid;
use crate::utils::uuid_to_string;

#[derive(Serialize)]
pub struct Transaction {
    #[serde(serialize_with = "uuid_to_string")]
    pub transaction_id: Uuid,
    pub bank_id: u8,
    pub customer_id: u16,
    pub amount: f32,
}