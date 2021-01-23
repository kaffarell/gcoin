mod blockchain;
mod payload;

use blockchain::block::*;
use blockchain::chain::*;

fn main() {
    let mut chain: Chain = Chain::new();
    let block: Block = Block::new();
    println!("{}", block);
    chain.add(block);

}
