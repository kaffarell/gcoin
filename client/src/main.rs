use std::io::stdin;
mod data;
mod console;
mod key;

fn main() {
    // Check if new public/private key or use existing one
    println!("Create new private/public key? \n[1] yes \n[2] no");
    let mut line = String::new();
    let _ = stdin().read_line(&mut line).unwrap();
    let choice_number: i32 = line.trim_end().parse().expect("Error parsing to i32");
    if choice_number == 1 {
        // Generate keys
        key::generate_keys();
    }else if choice_number == 2 {
        // Output public key
        println!("Your public key: \n{}", key::get_public_key());
        // Get Transaction


        // Sign
        let tran: data::Transaction = data::Transaction{sender: "204985".to_string(), receiver: "34573495".to_string(), amount: "3.4".to_string()};
        key::read_key(tran)
    }else{
        println!("Input error");
    }
}
