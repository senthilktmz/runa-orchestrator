use reqwest::blocking::Client;
use runautils::cipher_item::{encrypt_payload, get_decrypted_payload};


fn to_json_literal_string(payload: &str) -> String {
    let escaped_payload = payload.replace("\"", "\\\"");
    format!("\"{}\"", escaped_payload)
}


#[test]
fn test_encrypt_and_decrypt_payload() {
    let test_key = b"0123456789abcdef0123456789abcdef";
    let associated_data = b"";
    let plaintext = r#"{
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

    // Encrypt the payload
    let encrypted_payload = encrypt_payload(test_key, plaintext.as_bytes(), associated_data)
        .expect("Encryption failed");

    let url = "http://127.0.0.1:9191/task_agent";
    let client = Client::new();

    let formatted_body = to_json_literal_string(encrypted_payload.as_str()); //  format!("\"{}\"", encrypted_payload);

    // Send the payload as a JSON string
    let response = client
        .post(url)
        .header("Content-Type", "application/json")
        .body(formatted_body) // Wrap in quotes to make it a JSON string
        .send();

    match response {
        Ok(res) if res.status().is_success() => {
            let response_text = res.text().expect("Failed to read response text");
            println!("Server Response: {}", response_text);

            // Decrypt the payload
            let decrypted_payload =
                get_decrypted_payload(response_text, test_key).expect("Decryption failed");
            println!("Decrypted Payload: {}", decrypted_payload);

            assert_eq!(
                decrypted_payload, plaintext,
                "Decrypted payload does not match original plaintext"
            );
        }
        Ok(res) => {
            eprintln!(
                "HTTP error! Status: {}, Details: {}",
                res.status(),
                res.text().unwrap_or_else(|_| "Unknown error".to_string())
            );
        }
        Err(e) => {
            eprintln!("Failed to send POST request: {}", e);
        }
    }
}
