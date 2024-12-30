use runautils::actix_server_util::{post_http_request};

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