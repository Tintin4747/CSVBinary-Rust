use transaction_processor::models::Transaction;
use std::fs;
use std::fs::File;
use std::io::{self, Write, BufWriter};
use rand::{Rng, rng};
use rand::distr::Uniform;
use uuid::Uuid;

fn main() -> io::Result<()> {
    let num_transactions = 10_000;

    fs::create_dir_all("../../out")?;

    // Fichier CSV
    let csv_file = File::create("../../out/transactions.csv")?;
    let mut csv_writer = csv::Writer::from_writer(csv_file);

    let binary_file = File::create("../../out/transactions.bin")?;
    let mut binary_writer = BufWriter::new(binary_file);

    let mut rng = rng();

    // Vous devez appeler .unwrap() sur les Uniform dans rand 0.9.1
    let bank_id_range = Uniform::new_inclusive(1, 115).unwrap();
    let customer_id_range = Uniform::new_inclusive(1, 1366).unwrap();
    let amount_range = Uniform::new_inclusive(-10_000.0_f32, 10_000.0_f32).unwrap();

    for _ in 0..num_transactions {
        let transaction = Transaction {
            transaction_id: Uuid::new_v4(),
            bank_id: rng.sample(bank_id_range),
            customer_id: rng.sample(customer_id_range),
            amount: (rng.sample(amount_range) * 100.0).round() / 100.0,
        };


        // Écriture dans le fichier CSV
        csv_writer.serialize(&transaction)?;

        // Écriture dans le fichier binaire (.bin)
        binary_writer.write_all(transaction.transaction_id.as_bytes())?;
        binary_writer.write_all(&transaction.bank_id.to_ne_bytes())?;
        binary_writer.write_all(&transaction.customer_id.to_ne_bytes())?;
        binary_writer.write_all(&transaction.amount.to_ne_bytes())?;
    }

    // Finalisation des fichiers
    csv_writer.flush()?;
    binary_writer.flush()?;

    println!("Fichiers transactions.csv et transactions.bin générés avec succès !");
    Ok(())
}