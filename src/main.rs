mod blockchain;

use blockchain::block::*;
use blockchain::chain::*;

fn main() {
    println!("Hello, world!");
    let vec: Vec<Block> = Vec::new();
    let chain1 =  Chain {
        chain: vec,
    };
}
