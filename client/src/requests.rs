use crate::data::Transaction;

pub fn get_balance(pub_key: String) -> Result<i32, reqwest::Error> {
    println!("tes");
    let body = reqwest::blocking::get("http://localhost:8000/chain").unwrap().text()?;
    println!("Body: {}", body);
    return Ok(0);
}

pub fn send_transaction(transaction: &Transaction) {
    let string = serde_json::to_string(transaction).unwrap();
    println!("{}", string);
}