mod blockchain;
mod payload;
mod utils;

use blockchain::block::*;
use blockchain::chain::*;
use utils::crypto::*;
use payload::data::Data;

fn main() {
    let mut chain: Chain = Chain::new();
    let block: Block = Block::new();
    let mut block1: Block = Block::new();
    let mut data: Vec<Data> = Vec::new();
    data.push(Data{ data_type: "damian -> lukas ".to_string()});
    data.push(Data{ data_type: "gabriel -> damian".to_string()});
    block1.initialize(data, "345.345345-34".to_string());
    block1.mine();
    hash_md5(format!("{}", block1));
    chain.add(block1);
}
