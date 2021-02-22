#![feature(proc_macro_hygiene, decl_macro)]

mod blockchain;
mod payload;
mod utils;
mod db;

use blockchain::block::*;
use blockchain::chain::*;
use payload::data::Transaction;
use chrono::prelude::*;

#[macro_use] extern crate rocket;

#[post("/add", data = "<transaction>")]
fn add_transaction(transaction: String) {
    let mut chain: Chain = Chain::new();
    let mut block: Block = Block::new();
    let mut data: Vec<Transaction> = Vec::new();
    let t: Transaction = serde_json::from_str(&transaction).unwrap();
    data.push(t);
    // Get time
    let time: DateTime<Local> = Local::now();
    block.initialize(data, time.to_string());
    block.mine();
    chain.add(block);
    chain.print();
}

fn main() {
    let t = Transaction{sender: "065sjdfsdf45".to_string(), receiver: "34h3453h345".to_string(), amount: "4.56".to_string()};
    println!("{}", serde_json::to_string(&t).unwrap());
    rocket::ignite().mount("/", routes![add_transaction]).launch();
}
