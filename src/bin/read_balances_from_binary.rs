use std::fs::File;
use std::io::{self, BufReader, Read};
use clap::Parser;
use transaction_processor::models::FileArgs;

fn main() -> io::Result<()> {
    let args = FileArgs::parse();
    
    let file_path_arg = args.file_path;
    let file_path: String = file_path_arg.unwrap_or("out/balances_by_user.bin".to_string());
    
    let file = File::open(file_path)?;
    let mut reader = BufReader::new(file);
    let mut customer_id_buffer = [0; 2];
    let mut balance_buffer = [0; 4];
    println!("Liste des soldes par utilisateur :");
    while reader.read_exact(&mut customer_id_buffer).is_ok() {
        reader.read_exact(&mut balance_buffer)?;
        let customer_id = u16::from_ne_bytes(customer_id_buffer);
        let balance = f32::from_ne_bytes(balance_buffer);
        println!("Customer ID: {}, Balance: {}", customer_id, balance);
    }
    Ok(())
}
