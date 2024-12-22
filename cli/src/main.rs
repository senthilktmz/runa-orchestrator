mod aes_gcm_util;
use aes_gcm_util::{encrypt, decrypt};
use crate::aes_gcm_util::aes_gcm_key_from_string_literal;
use base64::{engine::general_purpose, Engine};

fn main() {
    encrypt_then_to_b64_string_test();
}

fn encrypt_then_to_b64_string_test() {
    println!("Running test {:?}", "encrypt_then_to_b64_string_test");
    let payload = r#"{
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

    let associated_data = b"sksks";
    let plaintext = payload.as_bytes();
    let key  = aes_gcm_key_from_string_literal(b"0123456789abcdef0123456789abcdef");
    let (ciphered, nonce) = encrypt(&key, payload.as_bytes(), associated_data);
    let b64_string = general_purpose::STANDARD.encode(ciphered);
    println!("Encrypted data as b64 : {:?}", b64_string);
}