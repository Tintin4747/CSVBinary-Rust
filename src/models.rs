use crate::utils::uuid_to_string;
use clap::Parser;
use serde::Serialize;
use uuid::Uuid;

/// Transaction structure
#[derive(Serialize)]
pub struct Transaction {
    #[serde(serialize_with = "uuid_to_string")]
    pub transaction_id: Uuid,
    pub bank_id: u8,
    pub customer_id: u16,
    pub amount: f32,
}

/// Arguments for reading binary files
#[derive(Parser)]
pub struct FileArgs {
    #[clap(long)]
    pub file_path: Option<String>,
}