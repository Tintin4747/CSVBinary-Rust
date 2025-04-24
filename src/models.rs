use serde::Serialize;
use uuid::Uuid;
use crate::utils::uuid_to_string;

#[derive(Serialize)]
pub(crate) struct Transaction {
    #[serde(serialize_with = "uuid_to_string")]
    pub(crate) transaction_id: Uuid,
    pub(crate) bank_id: u8,
    pub(crate) customer_id: u16,
    pub(crate) amount: f32,
}