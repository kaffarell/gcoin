use crate::blockchain::block;

pub struct Chain {
    pub chain: Vec<block::Block>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            chain: Vec::new()
        }
    }

    pub fn add(mut self, block: block::Block) {
        self.chain.push(block);
    }
}