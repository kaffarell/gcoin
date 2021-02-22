use std::io;
use crate::data::Transaction;

pub fn get_transaction(sender: String) -> Transaction {
    println!("Enter public key recipient:");
    let mut line = String::new();
    let b = io::stdin().read_line(&mut line).unwrap();
    println!("Enter amount:");
    let mut line1 = String::new();
    let b1 = io::stdin().read_line(&mut line1).unwrap();
    return Transaction{sender: sender, receiver: line, amount: line1};
}