use crate::payload::data::Data;
use crate::utils::crypto;

#[derive(Hash)]
pub struct Block {
    hash: String,
    prev_hash: String,
    data: Vec<Data>,
    // Fix date type
    date: String,
    nonce: i64,
}

impl Block {
    pub fn new() -> Block {
        Block {
            hash: String::new(),
            prev_hash: String::new(),
            data: Vec::new(),
            date: String::new(),
            nonce: 0,
        }
    }

    pub fn initialize(&mut self, data: Vec<Data>, date: String) {
        self.data = data;
        self.date = date;
    }

    pub fn mine(&mut self){
        let mut nonce: i64 = 0;
        let mut blockString = self.generate_string();
        let mut hash = blockString.as_bytes();
        println!("Crypto function 2");
        crypto::hash_md5(hash);
        let difficulty: usize = 5;
        while &hash[..difficulty] != "00000".as_bytes() {
            nonce = nonce + 1;
            blockString = self.generate_string();
            hash = crypto::hash_md5(blockString).as_bytes();
        }
    }

    pub fn generate_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.hash);
        s.push_str(&self.prev_hash);
        s.push_str(&format!("{:?}", &self.data)[..]);
        return s;
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\nhash: {}, \nprev_hash: {}, \ndata: {:?}, \ndate: {}, \nnonce: {}\n)", self.hash, self.prev_hash, self.data, self.date, self.nonce)
    }
}