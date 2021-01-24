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
    block1.initialize("dfasdfasdfasdf".to_string(), "fajsdölfjasdlfkj".to_string(), Data{ data_type: "dadfa".to_string()}, "345.345345-34".to_string(), 4567);
    hash_md5(format!("{}", block));
    hash_md5(format!("{}", block1));
    println!("{}", block);
    println!("{}", block1);
    chain.add(block);
}
