use crate::data;

pub struct Block {
    hash: String,
    prev__hash: String,
    data: data::Data,
    // Fix date type
    date: String,
    nonce: i64,
}