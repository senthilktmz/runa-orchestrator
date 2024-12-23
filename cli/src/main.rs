mod aes_gcm_util;
use std::error::Error;
use crate::aes_gcm_util::{aes_gcm_key_from_string_literal, AesGcmKey};
use aes_gcm::aead::consts::U12;
use aes_gcm_util::{decrypt, encrypt};
use base64::{engine::general_purpose, Engine};
use hex;
use serde::Serialize;
use aes_gcm::aead::{Aead, Payload};
use aes_gcm::{
    aead::{AeadCore, AeadInPlace, KeyInit, OsRng},
    Aes256Gcm, Key, Nonce,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bash_script_run_test_01()?;
    Ok(())
}

#[derive(Serialize)]
struct CipherItem<T> {
    ciphertext: T,
    nonce: T,
    associated_data: T
}


impl CipherItem<String> {
}

impl CipherItem<String> {
    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json_string = serde_json::to_string(&self)?;
        Ok(json_string)
    }
    pub fn decrypt(&self, key: Key<Aes256Gcm>) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let nonce_bytes = general_purpose::STANDARD.decode(&self.nonce)?;
        let nonce = Nonce::from_slice(&nonce_bytes);
        let ciphertext = general_purpose::STANDARD.decode(&self.ciphertext)?;
        let associated_data = self.associated_data.as_bytes();
        let cipher = Aes256Gcm::new(&key);
        let decrypted_bytes = cipher
            .decrypt(
                nonce,
                Payload {
                    msg: &ciphertext,
                    aad: associated_data,
                },
            )
            .map_err(|e| format!("Decryption failed: {:?}", e))?;
        Ok(decrypted_bytes)
    }
}

pub fn encrypt_bytes(
    key: Key<Aes256Gcm>,
    plaintext: &[u8],
    associated_data: &[u8],
) -> CipherItem<String> {

    let cipher = Aes256Gcm::new(&key);
    let nonce = Aes256Gcm::generate_nonce(&mut OsRng); // 96-bits; unique per encryption

    let encrypted_bytes = cipher
        .encrypt(
            &nonce,
            Payload {
                msg: plaintext,
                aad: associated_data,
            },
        )
        .expect("Encryption failed");

    let ciphertext_b64 = general_purpose::STANDARD.encode(encrypted_bytes);
    let nonce_b64 = general_purpose::STANDARD.encode(nonce);

    CipherItem {
        ciphertext: ciphertext_b64,
        nonce: nonce_b64,
        associated_data: general_purpose::STANDARD.encode(associated_data),
    }
}

fn bash_script_run_test_01() -> Result<(), Box<dyn std::error::Error>> {
    let key = <Key<Aes256Gcm>>::from(aes_gcm_key_from_string_literal(b"0123456789abcdef0123456789abcdef"));
    let ci = encrypt_bytes(<Key<Aes256Gcm>>::from(key),
             RUN_BASH_SCRIPT_PAYLOAD01.as_bytes(), b"");
    println!("jjjjjj");
    let json_str = ci.to_json()?;
    println!("{}", json_str);

    let d = ci.decrypt(key)?;
    println!("kkkkkkkk");
    println!("{}", String::from_utf8(d)?);

    Ok(())
}




fn encrypt_then_to_b64_string_test(
    payload: &str,
) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
    println!("Running test {:?}", "encrypt_then_to_b64_string_test");
    let associated_data = b"sksks";
    let plaintext = payload.as_bytes();
    let key = aes_gcm_key_from_string_literal(b"0123456789abcdef0123456789abcdef");
    let (ciphered, nonce) = encrypt(&key, payload.as_bytes(), associated_data);
    let b64_string = general_purpose::STANDARD.encode(ciphered);
    println!("Encrypted data as b64 : {:?}", b64_string);
    let nonce_str = hex::encode(nonce);
    println!("nonce data : {:?}", nonce_str);

    let nonce_vec8 = hex::decode(nonce_str)?;
    let reconstructed_nonce = Nonce::<U12>::from_slice(nonce_vec8.as_slice());
    Ok((b64_string, nonce_vec8))
}

const RUN_BASH_SCRIPT_PAYLOAD01: &str = r#"{
    "request_params": {
        "request_type": "command_execution",
        "command_params": {
            "command_type": "run_bash_script",
            "run_mode": "async",
            "command_data": {
                "run_bash_script_data": {
                    "script_data": "ZWNobyAnSGVsbG8sIFdvcmxkIScK",
                    "script_data_type": "bash_script_b64_utf8"
                }
            },
            "command_progress_info_params": {
                "stream_progress_type": "realtime"
            }
        }
    }
}"#;
//
//
