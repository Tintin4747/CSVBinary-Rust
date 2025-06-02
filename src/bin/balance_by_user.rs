use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};
use transaction_processor::utils::open_transaction_file;

fn main() -> io::Result<()> {
    // Lecture du fichier Binaire
    let binary_file = open_transaction_file();

    // Initialisation du lecteur binaire
    let mut reader = io::BufReader::new(binary_file);
    let mut buffer = [0; 16]; // 16 bytes for UUID
    let mut bank_id_buffer = [0; 1]; // 1 byte for bank_id
    let mut customer_id_buffer = [0; 2]; // 2 bytes for customer_id
    let mut amount_buffer = [0; 4]; // 4 bytes for amount

    let balance_out_file = File::create("out/balances_by_user.bin")?;
    let mut balance_writer = BufWriter::new(balance_out_file);

    let mut old_customer_id: u16 = 0;
    let mut current_customer_id: u16;
    let mut current_balance: f32 = 0.0;

    while reader.read_exact(&mut buffer).is_ok() {
        reader.read_exact(&mut bank_id_buffer)?;
        reader.read_exact(&mut customer_id_buffer)?;
        current_customer_id = u16::from_ne_bytes(customer_id_buffer);
        reader.read_exact(&mut amount_buffer)?;
        if current_customer_id == old_customer_id {
            current_balance = current_balance + f32::from_ne_bytes(amount_buffer);
        } else {
            // Écriture du solde initial dans le fichier de sortie
            balance_writer.write_all(&current_customer_id.to_ne_bytes()).expect("Erreur lors de l'écriture du customer_id");
            balance_writer.write_all(&current_balance.to_ne_bytes()).expect("Erreur lors de l'écriture du solde");
            old_customer_id = current_customer_id;
            current_balance = 0.0;
        }
    }

    // Fermeture des fichiers
    balance_writer
        .flush()
        .expect("Erreur lors de la finalisation du fichier 'balances_updated.bin'");
    println!("Fichier balances_by_user.bin généré avec succès !");
    Ok(())
}
