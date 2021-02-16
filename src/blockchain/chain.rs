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

    pub fn add(&mut self, mut block: block::Block) {
        if self.chain.len() >= 1 {
            block.prev_hash = self.chain[self.chain.len()-1].hash.clone();
        }else{
            // Genesis Block has to be made prior
            block.prev_hash = "00000000".to_string();
        }
        self.chain.push(block);
    }

    pub fn validate(&self){
        
    }
}