use crate::payload::data::Transaction;
use crate::utils::crypto;
use serde::{Serialize, Deserialize};


#[derive(Hash)]
#[derive(Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub prev_hash: String,
    pub height: i64,
    pub data: Vec<Transaction>,
    // Fix date type
    pub date: String,
    pub nonce: i64,
}

impl Block {
    pub fn new() -> Block {
        Block {
            hash: String::new(),
            prev_hash: String::new(),
            height: 0,
            data: Vec::new(),
            date: String::new(),
            nonce: 0,
        }
    }

    pub fn initialize(&mut self, data: Vec<Transaction>, date: String) {
        self.data = data;
        self.date = date;
    }

    pub fn mine(&mut self){
        let mut nonce: i64 = 0;
        self.nonce = nonce;
        let mut block_string: String;
        block_string = self.generate_string();
        let mut hash = block_string.as_bytes();
        crypto::hash_md5(hash);

        // Mining difficulty
        let difficulty: usize = 5;

        let mut created_hash: String = String::new();
        while &hash[..difficulty] != "00000".as_bytes() {
            nonce = nonce + 1;
            self.nonce = nonce;
            block_string = self.generate_string();
            created_hash = crypto::hash_md5(block_string);
            hash = created_hash.as_bytes();
        }
        self.hash = created_hash;
        self.nonce = nonce;
        // Mining reward
        // TODO: Find out which address is the miner
        let t = Transaction{sender: "".to_string(), receiver: "MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDFwGJPY8CH4cHoo7KN8EAcBAvy0Y6/HGDjdxPpGohwO0G1Zt0jY+ps29Kd8aX6QwPgnMkWOudeXufi01Y6UYWiWh774d+6H1446+xl7gAG7rWdchygRM0fPV12eiNJDOtcC7R0S06Ez0fZEJ4o2RwFXLoKjI/7vJPGpJQ8Sbx6fwIDAQAB".to_string(), amount: "1".to_string(), signature: vec![0, 0, 0]};
        self.data.push(t);
    }

    pub fn generate_string(&self) -> String {
        let mut s = String::new();
        s.push_str(&self.hash);
        s.push_str(&self.prev_hash);
        s.push_str(&self.height.to_string());
        s.push_str(&format!("{:?}", &self.data)[..]);
        s.push_str(&self.date);
        s.push_str(&self.nonce.to_string());
        return s;
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\nhash: {}, \nprev_hash: {}, \nheight: {}, \ndata: {:?}, \ndate: {}, \nnonce: {}\n)", self.hash, self.prev_hash, self.height, self.data, self.date, self.nonce)
    }
}