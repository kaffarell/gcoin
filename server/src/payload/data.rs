use serde::{Serialize, Deserialize};


#[derive(Hash)]
#[derive(Serialize, Deserialize)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: String,
    pub signature: Vec<u8>,
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
