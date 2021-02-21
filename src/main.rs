#![feature(proc_macro_hygiene, decl_macro)]

mod blockchain;
mod payload;
mod utils;
mod db;

use blockchain::block::*;
use blockchain::chain::*;
use payload::data::Data;
use chrono::prelude::*;

#[macro_use] extern crate rocket;

#[post("/add", data = "<transaction>")]
fn add_transaction(transaction: String) {
    let mut chain: Chain = Chain::new();
    let mut block: Block = Block::new();
    let mut data: Vec<Data> = Vec::new();
    data.push(Data{ data_type: transaction});
    // Get time
    let time: DateTime<Local> = Local::now();
    block.initialize(data, time.to_string());
    block.mine();
    chain.add(block);
}

fn main() {
    rocket::ignite().mount("/", routes![add_transaction]).launch();
}
