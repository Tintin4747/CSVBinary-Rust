use std::fs::File;
use uuid::Uuid;

pub(crate) fn uuid_to_string<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&uuid.to_string())
}

pub fn open_transaction_file() -> File {
    if std::path::Path::new("out/transactions-sorted.bin").exists() {
        File::open("out/transactions-sorted.bin").unwrap()
    } else {
        File::open("out/transactions.bin").unwrap()
    }
}