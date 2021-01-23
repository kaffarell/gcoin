use crate::payload::data::Data;

pub struct Block {
    hash: String,
    prev_hash: String,
    data: Data,
    // Fix date type
    date: String,
    nonce: i64,
}

impl Block {
    pub fn new() -> Block {
        Block {
            hash: String::new(),
            prev_hash: String::new(),
            data: Data{data_type: String::new()},
            date: String::new(),
            nonce: 0,
        }
    }

    pub fn initialize(mut self, hash: String, prev_hash: String, data: Data, date: String, nonce: i64) {
        self.hash = hash;
        self.prev_hash = prev_hash;
        self.data = data;
        self.date = date;
        self.nonce = nonce;
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\nhash: {}, \nprev_hash: {}, \ndata: {}, \ndate: {}, \nnonce: {}\n)", self.hash, self.prev_hash, self.data, self.date, self.nonce)
    }
}