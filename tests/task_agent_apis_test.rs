use runautils::http_calls::post_http_request;
use serde::{Deserialize, Serialize};

#[test]
fn test_add_task_agent() {
    let test_key = b"0123456789abcdef0123456789abcdef";
    let associated_data = b"";
    let plaintext = r#"{
    "request_params": {
        "request_type": "task_agent",
        "command_params": {
            "command_type": "add_task_agent",
            "command_data": {
                "caller_id": "task_agent_self",
                "ip_or_name": "127.0.0.1",
                "protocol": ["http", "https"],
                "port": "9000",
                "os_type": "linux",
                "arch_type": "amd64",
                "os_version": "ubuntu/20.04",
                "tags": {
                    "memory": "64gb",
                    "num_cores": "4"
                }
            }
        }
    }
}"#;

    let params = TaskAgentParams {
        caller_id: "task_agent_self",
        ip_or_name: "127.0.0.1",
        protocol: r#"["http", "https"]"#,
        port: "9090",
        os_type: "linux",
        arch_type: "amd64",
        os_version: "ubuntu/20.04",
        memory: "64gb",
        num_cores: "4",
    };

    let json_str = get_json_str(params);

    let url = "http://127.0.0.1:9191/task_agent";
    let response = post_http_request(url, json_str.as_str(), test_key, associated_data).unwrap();

    assert!(response.status().is_success());
}

#[derive(Debug)]
struct TaskAgentParams<'a> {
    caller_id: &'a str,
    ip_or_name: &'a str,
    protocol: &'a str,
    port: &'a str,
    os_type: &'a str,
    arch_type: &'a str,
    os_version: &'a str,
    memory: &'a str,
    num_cores: &'a str,
}

fn get_json_str(params: TaskAgentParams) -> String {
    format!(
        r#"{{
            "request_params": {{
                "request_type": "task_agent",
                "command_params": {{
                    "command_type": "add_task_agent",
                    "command_data": {{
                        "caller_id": "{}",
                        "ip_or_name": "{}",
                        "protocol": {},
                        "port": "{}",
                        "os_type": "{}",
                        "arch_type": "{}",
                        "os_version": "{}",
                        "tags": {{
                            "memory": "{}",
                            "num_cores": "{}"
                        }}
                    }}
                }}
            }}
        }}"#,
        params.caller_id,
        params.ip_or_name,
        params.protocol,
        params.port,
        params.os_type,
        params.arch_type,
        params.os_version,
        params.memory,
        params.num_cores
    )
}
