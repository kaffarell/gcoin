use crate::data::Transaction; 
use crate::data::Block;


pub fn get_balance(pub_key: String) -> Result<i32, reqwest::Error> {
    println!("pub_key: {}", pub_key);
    let body: String = reqwest::blocking::get("http://localhost:8000/chain").unwrap().text()?;
    let mut body_string: String = body.clone();

    // Store balance
    let mut balance: f32 = 0.0;
    //println!("{}", body);
    loop {
        // Search for start of first block
        let start_index = body_string.find("{\"hash\"");
        if start_index.is_none() {
            break;
        }
        // Remove already found first occurence of {"hash
        body_string = body_string.replacen("{\"hash\"", "{\"xxxx\"", 1);
        // Search for start of second block
        let mut end_index = body_string.find("{\"hash\"");
        if end_index.is_none() {
            end_index = Some(body.len());
        }
        let block_as_string = body[start_index.unwrap()..end_index.unwrap()].to_string();

        let block: Block = serde_json::from_str(&block_as_string[..]).expect("Error parsing block json");
        for i in 0..block.data.len() {
            let receiver = block.data[i].receiver.replace("\n", "").replace("\r", "");
            let sender = block.data[i].sender.replace("\n", "").replace("\r", "");
            let amount = block.data[i].amount.replace("\n", "").replace("\r", "");

            if receiver == pub_key {
                println!(" + {}", amount.parse::<f32>().unwrap());
                balance = balance + amount.parse::<f32>().unwrap();
            }else if sender == pub_key {
                println!(" - {}", amount.parse::<f32>().unwrap());
                balance = balance - amount.parse::<f32>().unwrap();
            }
        }
    }
    println!("Balance: {}", balance);
    return Ok(0);
}

pub fn send_transaction(transaction: &Transaction) {
    let string = serde_json::to_string(transaction).unwrap();
    let client = reqwest::blocking::Client::new();
    let res = client.post("http://localhost:8000/add").body(string).send().unwrap();
    println!("{:?}", res);
}