use std::collections::{BTreeMap, HashMap};
use std::io;
use std::io::Read;
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
    
    // Création d'un fichier CSV pour stocker les soldes
    let csv_file = std::fs::File::create("out/balances_by_user_bin.csv")?;
    let mut csv_writer = csv::Writer::from_writer(csv_file);
    // Écriture des soldes dans le fichier CSV
    for (customer_id, balance) in &sorted_balances {
        csv_writer.write_record(&[customer_id.to_string(), balance.to_string()])?;
    }
    // Finalisation du fichier CSV
    csv_writer.flush()?;
    println!("Fichier balances_by_user.csv généré avec succès !");
    Ok(())
}