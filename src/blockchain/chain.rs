use crate::blockchain::block;
use crate::payload::data::Data;

pub struct Chain {
    pub chain: Vec<block::Block>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            chain: Vec::new()
        }
    }

    pub fn add(&mut self, block: block::Block) {
        self.chain.push(block);
    }

    pub fn validate(&self){
        
    }

    pub fn mine(&self, data: Vec<Data>) {
        // Create Block
        
    }
}