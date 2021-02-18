use serde::{Serialize, Deserialize};


#[derive(Hash)]
#[derive(Serialize, Deserialize)]
pub struct Data {
    pub data_type: String,

}


impl std::fmt::Display for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\ndata_type: {}\n)", self.data_type)
    }
}

impl std::fmt::Debug for Data {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(\ndata_type: {}\n)", self.data_type)
    }
}