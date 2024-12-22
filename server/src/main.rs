
mod aes_gcm_util;

use crate::aes_gcm_util::{
    aes_gcm_key_from_string_literal, encrypt, decrypt, AesGcmKey,
};

fn main() {
    let associated_data = b"sksks";
    let data = b"jsjsjs";
    let key  = aes_gcm_key_from_string_literal(b"0123456789abcdef0123456789abcdef");
    println!("Original data: {:?}", data);
    let (ciphered, nonce) =  encrypt(&key,   data, b"sksks");
    println!("Encrypted data: {:?}", ciphered);
    let plain_text = decrypt(&key, &nonce, &ciphered, associated_data);
    println!("Decrypted data: {:?}", plain_text);
}

//
//