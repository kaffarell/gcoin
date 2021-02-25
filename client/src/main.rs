use std::io::stdin;
mod data;
mod console;
mod key;

fn main() {
    // Check if new public/private key or use existing one
    println!("Create new private/public key? \n[1] yes \n[2] no");
    let mut line = String::new();
    let b = stdin().read_line(&mut line).unwrap();
    let choice_number: i32 = line.trim_end().parse().expect("Error parsing to i32");
    if choice_number == 1 {
        // Generate public private key
        key::generate_keys("test".to_string());
    }else if choice_number == 2 {
        // Read public private key from file
    }else{
        println!("Input error");
    }
}
