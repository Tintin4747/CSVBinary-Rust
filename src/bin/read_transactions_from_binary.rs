use std::io;
use std::io::Read;
use transaction_processor::utils::open_transaction_file;

/// Lecture le fichier binaire et affiche les transactions
fn main() -> io::Result<()>{
    // Lecture du fichier Binaire
    let binary_file = open_transaction_file();
    
    // Créaction d'une liste pour stocker les transactions
    let mut transactions: Vec<transaction_processor::models::Transaction> = Vec::new();
    let mut reader = std::io::BufReader::new(binary_file);
    let mut buffer = [0; 16]; // 16 bytes for UUID
    let mut bank_id_buffer = [0; 1]; // 1 byte for bank_id
    let mut customer_id_buffer = [0; 2]; // 2 bytes for customer_id
    let mut amount_buffer = [0; 4]; // 4 bytes for amount
    while reader.read_exact(&mut buffer).is_ok() {
        let transaction_id = uuid::Uuid::from_slice(&buffer).unwrap();
        reader.read_exact(&mut bank_id_buffer)?;
        let bank_id = u8::from_ne_bytes(bank_id_buffer);
        reader.read_exact(&mut customer_id_buffer)?;
        let customer_id = u16::from_ne_bytes(customer_id_buffer);
        reader.read_exact(&mut amount_buffer)?;
        let amount = f32::from_ne_bytes(amount_buffer);

        // Créer une nouvelle transaction et l'ajouter à la liste
        let transaction = transaction_processor::models::Transaction {
            transaction_id,
            bank_id,
            customer_id,
            amount,
        };
        transactions.push(transaction);
    }
    
    // Trier par transaction_id
    transactions.sort_by(|a, b| a.transaction_id.cmp(&b.transaction_id));
    
    // Afficher les transactions
    for transaction in transactions.iter() {
        println!("Transaction ID: {}, Bank ID: {}, Customer ID: {}, Amount: {}", 
            transaction.transaction_id, 
            transaction.bank_id, 
            transaction.customer_id, 
            transaction.amount);
    }
    println!();
    Ok(())
}