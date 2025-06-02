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
    
    let transactions_file = File::open("out/transactions-sorted.bin")
        .or_else(|_| File::open("out/transactions.bin"))
        .expect("Erreur lors de l'ouverture du fichier 'transactions.bin' ou 'transactions-sorted.bin'");
    
    let mut balance_reader = BufReader::new(balance_file);
    let mut customer_id_buffer = [0; 2]; // 2 bytes for customer_id
    let mut balance_buffer = [0; 4]; // 4 bytes for balance
    
    let mut transactions_reader = BufReader::new(transactions_file);
    let mut uuid_buffer = [0; 16]; // 16 bytes for UUID
    let mut bank_id_buffer = [0; 1]; // 1 byte for bank_id
    let mut amount_buffer = [0; 4]; // 4 bytes for amount
    
    let balance_out_file = File::create("out/balances_updated.bin")
        .expect("Erreur lors de la création du fichier 'balances_updated.bin'");
    let mut balance_writer = BufWriter::new(balance_out_file);
    
    let mut current_customer_id: u16;
    let mut current_balance: f32;
    
    while balance_reader.read_exact(&mut customer_id_buffer).is_ok() {
        balance_reader.read_exact(&mut balance_buffer).expect("Erreur lors de la lecture du solde");
        current_customer_id = u16::from_ne_bytes(customer_id_buffer);
        current_balance = f32::from_ne_bytes(balance_buffer);
        
        // Tant que le prochaine transaction a le même customer_id, on met à jour le solde
        while transactions_reader.read_exact(&mut uuid_buffer).is_ok() {
            transactions_reader.read_exact(&mut bank_id_buffer).expect("Erreur lors de la lecture du bank_id");
            transactions_reader.read_exact(&mut customer_id_buffer).expect("Erreur lors de la lecture du customer_id");
            let transaction_customer_id = u16::from_ne_bytes(customer_id_buffer);
            transactions_reader.read_exact(&mut amount_buffer).expect("Erreur lors de la lecture du montant");
            let amount = f32::from_ne_bytes(amount_buffer);
            
            if transaction_customer_id == current_customer_id {
                // Mettre à jour le solde
                current_balance += amount;
            } else {
                break;
            }
        }
        
        // Écriture du solde initial dans le fichier de sortie
        balance_writer.write_all(&current_customer_id.to_ne_bytes()).expect("Erreur lors de l'écriture du customer_id");
        balance_writer.write_all(&current_balance.to_ne_bytes()).expect("Erreur lors de l'écriture du solde");
    }
    
    // Fermeture des fichiers
    balance_writer.flush().expect("Erreur lors de la finalisation du fichier 'balances_updated.bin'");
    println!("Le fichier 'balances_updated.bin' a été mis à jour avec succès !");
}

