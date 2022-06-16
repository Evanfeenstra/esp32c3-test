
use esp_idf_sys as _;
use secp256k1::{Message, PublicKey, Secp256k1, SecretKey};

fn main() {

    esp_idf_sys::link_patches();
    let secp = Secp256k1::new();

    let secret_key = SecretKey::from_slice(&[0xcd; 32]).expect("32 bytes, within curve order");
    let public_key = PublicKey::from_secret_key(&secp, &secret_key);
    
    let message = Message::from_slice(&[0xab; 32]).expect("32 bytes");

    let sig = secp.sign_ecdsa(&message, &secret_key);
    assert!(secp.verify_ecdsa(&message, &sig, &public_key).is_ok());

    println!("signature verified!");
    println!("Hello, world!");
}