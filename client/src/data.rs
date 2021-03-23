use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: String,
    pub signature: Vec<u8>,
}



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


impl std::fmt::Display for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\nsender: {}\nreceiver: {}\namount: {}\n)", 
        self.sender, self.receiver, self.amount)
    }
}

impl std::fmt::Debug for Transaction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\nsender: {}\nreceiver: {}\namount: {}\n)", 
        self.sender, self.receiver, self.amount)
    }
}
