mod blockchain;
mod payload;
mod utils;
mod db;

use blockchain::block::*;
use blockchain::chain::*;
use payload::data::Transaction;
use chrono::prelude::*;
use actix_web::{get, post, web, App, HttpResponse, HttpServer, Responder};
use actix::Arbiter;


#[post("/add")] 
async fn add_transaction(transaction: String) -> impl Responder{
    let chain: Chain = Chain::new();
    let mut block: Block = Block::new();
    let mut data: Vec<Transaction> = Vec::new();
    let t: Transaction = serde_json::from_str(&transaction).unwrap();
    data.push(t);
    // Get time
    let time: DateTime<Local> = Local::now();
    block.initialize(data, time.to_string());
    /*
    Arbiter::new().exec_fn(move || {
        async move {
            add_block_to_chain(block, chain).await;
        };
    });
    */
    let _myfuture = add_block_to_chain(block, chain).await;

    return HttpResponse::Ok().finish();
}

#[get("/chain")]
async fn get_chain() -> String {
    let chain: Chain = Chain::new();
    return chain.print();
}

async fn add_block_to_chain(mut block: Block, mut chain: Chain) {
    println!("Added blocks");
    block.mine();
    chain.add(block);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // TODO: remove this
    let t = Transaction{sender: "065sjdfsdf45".to_string(), receiver: "34h3453h345".to_string(), amount: "4.56".to_string(), signature: vec![0, 0, 0]};
    println!("{}", serde_json::to_string(&t).unwrap());

    HttpServer::new( || {
        App::new()
            .service(add_transaction)
            .service(get_chain)
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
