mod aes_gcm_util;

use std::error::Error;
use crate::aes_gcm_util::aes_gcm_key_from_string_literal;
use aes_gcm::aead::consts::U12;
use aes_gcm::Nonce;
use aes_gcm_util::{decrypt, encrypt};
use base64::{engine::general_purpose, Engine};
use hex;
use serde::Serialize;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    bash_script_run_test_01()?;
    Ok(())
}

#[derive(Serialize)]
struct CipherItem<T> {
    #[serde(skip_serializing)]
    original_text: T,
    #[serde(skip_serializing)]
    is_delete_original_string: bool,
    ciphertext: T,
    nonce: T,
    associated_data: T
}

impl CipherItem<String> {
    fn encrypt_text(&mut self) -> Result<(), Box<dyn Error>> {
        let payload = self.original_text.as_str();
        let associated_data = self.associated_data.as_bytes();
        let (ciphertext, nonce_str) = encrypt_payload(payload, associated_data)?;
        self.ciphertext = ciphertext;
        self.nonce = nonce_str;
        if self.is_delete_original_string {
            self.original_text.clear();
        }
        Ok(())
    }

    fn to_json(&self) -> Result<String, Box<dyn Error>> {
        let json_string = serde_json::to_string(&self)?;
        Ok(json_string)
    }

}

fn bash_script_run_test_01() -> Result<(), Box<dyn std::error::Error>> {

    let mut cipher_item = CipherItem {
        original_text: String::from(RUN_BASH_SCRIPT_PAYLOAD01),
        ciphertext: String::new(),
        nonce: String::new(),
        associated_data: String::from("lsisusu"),
        is_delete_original_string: false,
    };

    cipher_item.encrypt_text()?;
    let json_str = cipher_item.to_json()?;

    println!("lllllllllllllll");
    println!("{}", json_str);

    Ok(())
}


fn encrypt_payload(
    payload: &str,
    associated_data: &[u8]
) -> Result<(String, String), Box<dyn std::error::Error>> {
    println!("Running test {:?}", "encrypt_then_to_b64_string_test");
    //let associated_data = b"sksks";
    let plaintext = payload.as_bytes();
    let key = aes_gcm_key_from_string_literal(b"0123456789abcdef0123456789abcdef");
    let (ciphered, nonce) = encrypt(&key, payload.as_bytes(), associated_data);
    let b64_string = general_purpose::STANDARD.encode(ciphered);
    println!("Encrypted data as b64 : {:?}", b64_string);
    let nonce_str = hex::encode(nonce);
    // println!("nonce data : {:?}", nonce_str);
    //
    // let nonce_vec8 = hex::decode(nonce_str)?;
    // let reconstructed_nonce = Nonce::<U12>::from_slice(nonce_vec8.as_slice());
    Ok((b64_string, nonce_str))
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
