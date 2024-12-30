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

    let url = "http://127.0.0.1:9191/task_agent";
    let response = post_http_request(url, plaintext, test_key, associated_data).unwrap();

    assert!(response.status().is_success());

}

fn post_http_request(url :&str, plain_text_payload : &str,
                     key : &[u8; 32],
                     associated_data : &[u8] ) -> reqwest::Result<reqwest::blocking::Response> {

    // Encrypt the payload
    let encrypted_payload = encrypt_payload(key, plain_text_payload.as_bytes(), associated_data)
        .expect("Encryption failed");

    let url = "http://127.0.0.1:9191/task_agent";
    let client = Client::new();

    let formatted_body = to_json_literal_string(encrypted_payload.as_str()); //  format!("\"{}\"", encrypted_payload);

    client
        .post(url)
        .header("Content-Type", "application/json")
        .body(formatted_body) // Wrap in quotes to make it a JSON string
        .send()
}