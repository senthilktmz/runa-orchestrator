mod aes_gcm_util;

use crate::aes_gcm_util::aes_gcm_key_from_string_literal;
use aes_gcm::aead::consts::U12;
use aes_gcm::Nonce;
use aes_gcm_util::{decrypt, encrypt};
use base64::{engine::general_purpose, Engine};
use hex;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (b64_encrypted_payload, nonce_sv8) =
        encrypt_then_to_b64_string_test(RUN_BASH_SCRIPT_PAYLOAD01)?;
    Ok(())
}

fn encrypt_then_to_b64_string_test(
    PAYLOAD: &str,
) -> Result<(String, Vec<u8>), Box<dyn std::error::Error>> {
    println!("Running test {:?}", "encrypt_then_to_b64_string_test");
    let associated_data = b"sksks";
    let plaintext = PAYLOAD.as_bytes();
    let key = aes_gcm_key_from_string_literal(b"0123456789abcdef0123456789abcdef");
    let (ciphered, nonce) = encrypt(&key, PAYLOAD.as_bytes(), associated_data);
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
