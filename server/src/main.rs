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
async fn add_transaction(transaction: String) -> String{
    let chain: Chain = Chain::new();
    let mut block: Block = Block::new();
    let mut data: Vec<Transaction> = Vec::new();
    let t: Transaction = serde_json::from_str(&transaction).unwrap();
    data.push(t);
    // Get time
    let time: DateTime<Local> = Local::now();
    block.initialize(data, time.to_string());

    // Execute the mining and adding to blockchain async
    tokio::spawn(async move {
        add_block_to_chain(block, chain).await;
    });

    return "OK".to_string();
}

#[get("/chain")]
fn get_chain() -> String {
    let chain: Chain = Chain::new();
    return chain.print();
}

async fn add_block_to_chain(mut block: Block, mut chain: Chain) {
    println!("Added blocks");
    block.mine();
    chain.add(block);
}


#[tokio::main]
async fn main() { 
    // TODO: remove this
    let t = Transaction{sender: "065sjdfsdf45".to_string(), receiver: "34h3453h345".to_string(), amount: "4.56".to_string(), signature: vec![0, 0, 0]};
    println!("{}", serde_json::to_string(&t).unwrap());

    rocket::ignite().mount("/", routes![add_transaction, get_chain]).launch().await.ok();
}
