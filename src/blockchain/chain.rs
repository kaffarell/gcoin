use crate::blockchain::block;
use leveldb::database::Database;
use crate::db::db;

pub struct Chain {
    pub db: Database<i32>,
}

impl Chain {
    pub fn new() -> Chain {
        Chain {
            db: db::create_database(),
        }
    }

    pub fn add(&mut self, mut block: block::Block) {
        if db::get_total_height(&self.db) >= 1 {
            block.prev_hash = db::get(&self.db, (db::get_total_height(&self.db)-1) as i32).hash;
            block.height = db::get_total_height(&self.db);
        }else{
            // TODO: Genesis Block has to be generated prior to all other blocks
            block.prev_hash = "00000000".to_string();
        }
        db::put(&self.db, &block);
    }

    pub fn print(self) {
        println!("length: {}", db::get_total_height(&self.db));
        for i in 0..db::get_total_height(&self.db)-1 {
            println!("Block: {}", db::get(&self.db, i as i32));
        }
    }

    pub fn validate(&self){
        
    }
}