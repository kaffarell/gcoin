use md5;
use openssl::sign::{Verifier};
use openssl::rsa::Rsa;
use openssl::pkey::PKey;
use openssl::hash::MessageDigest;
use crate::payload::data::Transaction;

pub fn hash_md5<T: AsRef<[u8]>>(t: T) -> String {
    let digest = md5::compute(t);
    return format!("{:x}", digest);
}

pub fn verify_transaction(transaction: &Transaction) -> bool {
    println!("Verifying transaction");
    let mut pub_key_string: String = "-----BEGIN PUBLIC KEY-----\n".to_string();
    pub_key_string.push_str(&transaction.sender[..]);
    pub_key_string.push_str("\n-----END PUBLIC KEY-----");

    let pub_key = Rsa::public_key_from_pem(&pub_key_string.as_bytes()).expect("Error pem to public key");
    let keypair = PKey::from_rsa(pub_key).unwrap();

    let data = format!("{}", transaction);
    let signature = &transaction.signature;


    let mut verifier = Verifier::new(MessageDigest::sha256(), &keypair).unwrap();
    verifier.update(data.as_bytes()).unwrap();
    return verifier.verify(&signature[..]).unwrap();
}