mod blockchain;
mod payload;
mod utils;
mod db;

use blockchain::block::*;
use blockchain::chain::*;
use payload::data::Data;
use db::db::*;

fn main() {

    let mut chain: Chain = Chain::new();
    let mut block1: Block = Block::new();
    let mut block2: Block = Block::new();

    let mut data: Vec<Data> = Vec::new();
    data.push(Data{ data_type: "damian -> lukas ".to_string()});
    data.push(Data{ data_type: "gabriel -> damian".to_string()});

    let mut data1: Vec<Data> = Vec::new();
    data1.push(Data{ data_type: "lukas -> lukas ".to_string()});
    data1.push(Data{ data_type: "gabriel -> gabriel".to_string()});

    block1.initialize(data, "345.345345-34".to_string());
    block2.initialize(data1, "345.345345-33".to_string());
    block1.mine();
    block2.mine();

    chain.add(block1);
    chain.add(block2);
    chain.print();
}
