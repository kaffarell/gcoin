use openssl::rsa::{Rsa};
use openssl::pkey::PKey;
use openssl::sign::{Signer};
use openssl::hash::MessageDigest;
use std::fs;
use crate::data;

pub fn generate_keys(){
    let rsa = Rsa::generate(1024).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem().unwrap(); 
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    let private_key_string = String::from_utf8(private_key).unwrap();
    let public_key_string = String::from_utf8(public_key).unwrap();

    println!("{}", private_key_string);
    println!("{}", public_key_string);

    // Write to file
    fs::write("./priv.pem", private_key_string).expect("Unable to write public key file");
    fs::write("./pub.pem", public_key_string).expect("Unable to write private key file");
}

pub fn sign(transaction: &mut data::Transaction) {
    // To String
    let priv_data =  fs::read_to_string("./priv.pem").expect("Unable to read private key file");

    // To rsa
    let priv_rsa = Rsa::private_key_from_pem(&priv_data.as_bytes()).expect("Error pem to private key");
    let keypair = PKey::from_rsa(priv_rsa).unwrap();

    // Sign the data
    let data = format!("{}", transaction);
    let mut signer = Signer::new(MessageDigest::sha256(), &keypair).unwrap();
    signer.update(data.as_bytes()).unwrap();
    let signature = signer.sign_to_vec().unwrap();
    //println!("{}", String::from_utf8_lossy(&signature));
    transaction.signature = signature;
}

pub fn get_public_key() -> String {
    // To String
    let pub_data =  fs::read_to_string("./pub.pem").expect("Unable to read public key file");
    return pub_data;
}

pub fn get_private_key() -> String {
    // To String
    let priv_data =  fs::read_to_string("./priv.pem").expect("Unable to read private key file");
    return priv_data;
}