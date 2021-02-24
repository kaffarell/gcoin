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

    pub fn print(self) -> String{
        let mut string: String = String::new();
        for i in 0..db::get_total_height(&self.db) {
            string.push_str(&serde_json::to_string(&db::get(&self.db, i as i32)).unwrap());
        }
        return string;
    }

    pub fn validate(&self){
        
    }
}