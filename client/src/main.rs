use std::io::stdin;
mod data;
mod crypto;
mod requests;

fn main() {
    // Check if new public/private key or use existing one
    println!("[1] Create new pub/priv key pair \n[2] Make Transaction \n[3] Show Info");
    let mut line = String::new();
    let _ = stdin().read_line(&mut line).unwrap();
    let choice_number: i32 = line.trim_end().parse().expect("Error parsing to i32");
    if choice_number == 1 {
        // Generate keys
        crypto::generate_keys();
    }else if choice_number == 2 {
        // Output public key
        println!("Your public key: \n{}", crypto::get_public_key());

        // Get Transaction
        println!("Enter recipient: ");
        let mut recipient_input = String::new();
        let _ = stdin().read_line(&mut recipient_input).unwrap();
        println!("Enter amount: ");
        let mut amount_input = String::new();
        let _ = stdin().read_line(&mut amount_input).unwrap();

        // Check if user has enough gcoins


        // Sign
        // Slice public key out of wrapping BEGIN PUBLIC KEY
        let public_key_string = crypto::get_public_key()[27..crypto::get_public_key().len()-26].to_string();
        let mut tran: data::Transaction = data::Transaction{sender: public_key_string, receiver: recipient_input, amount: amount_input, signature: vec![0]};
        crypto::sign(&mut tran);

        // Ask for confermation
        println!("-----------------------------------------------");
        println!("{}", tran);
        println!("-----------------------------------------------");
        println!("Is this transaction correct? \n[1] yes \n[2] no");
        let mut transaction_confermation_input = String::new();
        let _ = stdin().read_line(&mut transaction_confermation_input).unwrap();
        let confermation: i32 = transaction_confermation_input.trim_end().parse().expect("Error parsing to i32");
        if confermation == 2 {
            return; 
        }

        // Send to server
        requests::send_transaction(&tran);

    }else if choice_number == 3 {
        println!("-----------------------------------------------");
        println!("{}", crypto::get_public_key()[27..crypto::get_public_key().len()-26].to_string().replace("\n", ""));
        println!("-----------------------------------------------");
        println!("Balance: ");
        requests::get_balance(crypto::get_public_key()).ok();
        println!("Public key:");
        println!("{}", crypto::get_public_key());
        println!("Private key:");
        println!("{}", crypto::get_private_key());
    }else{
        println!("Input error");
    }
}
