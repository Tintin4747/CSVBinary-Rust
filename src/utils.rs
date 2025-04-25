use uuid::Uuid;

pub(crate) fn uuid_to_string<S>(uuid: &Uuid, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    serializer.serialize_str(&uuid.to_string())
}

pub fn open_transaction_file() -> std::fs::File {
    if std::path::Path::new("out/transactions-filtered.bin").exists() {
        std::fs::File::open("out/transactions-filtered.bin").unwrap()
    } else {
        std::fs::File::open("out/transactions.bin").unwrap()
    }
}