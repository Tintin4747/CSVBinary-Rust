use std::collections::{HashMap, BTreeMap};
use std::fs::File;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::Path;

fn main() {
    if !Path::new("out/balances_by_user.bin").exists(){
        eprintln!("Le fichier 'out/balances_by_user.bin' n'existe pas. Veuillez d'abord exécuter le programme 'balance_by_user'.");
        std::process::exit(1);
    }
    let balance_file = File::open("out/balances_by_user.bin")
        .expect("Erreur lors de l'ouverture du fichier 'balances_by_user.bin'");
    
    let transactions_file = File::open("out/transactions-filtered.bin")
        .or_else(|_| File::open("out/transactions.bin"))
        .expect("Erreur lors de l'ouverture du fichier 'transactions.bin' ou 'transactions-filtered.bin'");
    
    let mut balance_reader = BufReader::new(balance_file);
    let mut customer_id_buffer = [0; 2]; // 2 bytes for customer_id
    let mut balance_buffer = [0; 4]; // 4 bytes for balance
    
    let mut transactions_reader = BufReader::new(transactions_file);
    let mut uuid_buffer = [0; 16]; // 16 bytes for UUID
    let mut bank_id_buffer = [0; 1]; // 1 byte for bank_id
    let mut amount_buffer = [0; 4]; // 4 bytes for amount
    
    let mut balances: HashMap<u16, f32> = HashMap::new();
    
    // 1. Charger balances_by_user.bin dans balances
    while balance_reader.read_exact(&mut customer_id_buffer).is_ok() {
        balance_reader.read_exact(&mut balance_buffer).expect("Erreur lecture balance");
        let customer_id = u16::from_ne_bytes(customer_id_buffer);
        let balance = f32::from_ne_bytes(balance_buffer);
        balances.insert(customer_id, balance);
    }

    // 2. Parcourir transactions et mettre à jour balances
    loop {
        // Lire UUID
        if transactions_reader.read_exact(&mut uuid_buffer).is_err() {
            break;
        }
        // Lire bank_id
        transactions_reader.read_exact(&mut bank_id_buffer).expect("Erreur lecture bank_id");
        // Lire customer_id
        transactions_reader.read_exact(&mut customer_id_buffer).expect("Erreur lecture customer_id");
        let customer_id = u16::from_ne_bytes(customer_id_buffer);
        // Lire amount
        transactions_reader.read_exact(&mut amount_buffer).expect("Erreur lecture amount");
        let amount = f32::from_ne_bytes(amount_buffer);
        // Mettre à jour balances
        let entry = balances.entry(customer_id).or_insert(0.0);
        *entry += amount;
    }

    // 3. Trier balances par customer_id
    let sorted_balances: BTreeMap<_, _> = balances.into_iter().collect();

    // 4. Écraser balances_by_user.bin avec les nouvelles données
    let bin_file = File::create("out/balances_by_user.bin").expect("Erreur création fichier balances_by_user.bin");
    let mut bin_writer = BufWriter::new(bin_file);
    for (customer_id, balance) in &sorted_balances {
        bin_writer.write_all(&customer_id.to_ne_bytes()).expect("Erreur écriture customer_id");
        bin_writer.write_all(&balance.to_ne_bytes()).expect("Erreur écriture balance");
    }
    bin_writer.flush().expect("Erreur flush fichier balances_by_user.bin");
    println!("Fichier balances_by_user.bin mis à jour avec succès !");
}

