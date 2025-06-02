use std::collections::{BTreeMap, HashMap};
use std::fs::File;
use std::io;
use std::io::{BufWriter, Read, Write};
use transaction_processor::utils::open_transaction_file;

fn main() -> io::Result<()>{
    // Lecture du fichier Binaire
    let binary_file = open_transaction_file();

    let mut balances: HashMap<u16, f32> = HashMap::new();

    let mut reader = io::BufReader::new(binary_file);
    let mut buffer = [0; 16]; // 16 bytes for UUID
    let mut bank_id_buffer = [0; 1]; // 1 byte for bank_id
    let mut customer_id_buffer = [0; 2]; // 2 bytes for customer_id
    let mut amount_buffer = [0; 4]; // 4 bytes for amount
    while reader.read_exact(&mut buffer).is_ok() {
        reader.read_exact(&mut bank_id_buffer)?;
        reader.read_exact(&mut customer_id_buffer)?;
        let customer_id = u16::from_ne_bytes(customer_id_buffer);
        reader.read_exact(&mut amount_buffer)?;
        let amount = f32::from_ne_bytes(amount_buffer);

        // Mettre à jour le solde dans la HashMap
        let balance = balances.entry(customer_id).or_insert(0.0);
        *balance += amount;
    }

    // Trier les soldes par ID client
    let sorted_balances: BTreeMap<_, _> = balances.into_iter().collect();

    // Création d'un fichier binaire pour stocker les soldes
    let bin_file = File::create("out/balances_by_user.bin")?;
    let mut bin_writer = BufWriter::new(bin_file);
    for (customer_id, balance) in &sorted_balances {
        bin_writer.write_all(&customer_id.to_ne_bytes())?;
        bin_writer.write_all(&balance.to_ne_bytes())?;
    }
    bin_writer.flush()?;
    println!("Fichier balances_by_user.bin généré avec succès !");
    Ok(())
}