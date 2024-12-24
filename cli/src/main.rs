//use base64::{engine::general_purpose, Engine};
//use hex;
//use serde::{Deserialize, Serialize};
//use std::error::Error;
//
//use aes_gcm::aead::consts::U12;
//use aes_gcm::aead::{Aead, Payload};
//use aes_gcm::{
//    aead::{AeadCore, KeyInit, OsRng},
//    Aes256Gcm, Key, Nonce,
//};
//
mod cipher_item;
use crate::cipher_item::client_encrypt_and_server_decrypt_test;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    client_encrypt_and_server_decrypt_test()?;
    Ok(())
}

//