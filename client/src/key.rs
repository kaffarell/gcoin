use openssl::rsa::{Rsa, Padding};
use openssl::symm::Cipher;

pub fn generate_keys(passphrase: String){
    let rsa = Rsa::generate(1024).unwrap();
    let private_key: Vec<u8> = rsa.private_key_to_pem_passphrase(Cipher::aes_128_cbc(), 
                                    passphrase.as_bytes().unwrap());
    let public_key: Vec<u8> = rsa.public_key_to_pem().unwrap();

    println!("Private key: {}", String::from_utf8(private_key).unwrap());
    println!("Public key: {}", String::from_utf8(public_key).unwrap());
}