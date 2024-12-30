use serde_json::Value;

pub fn process_add_task_agent(command_params: &Value) -> Result<(), String> {

    let command_data = command_params.
        get("command_data")
        .ok_or("Missing command_data")?;

    let ip_or_name = command_data.get("ip_or_name").and_then(|v| v.as_str()).unwrap_or("");
    let caller_id = command_data.get("caller_id").and_then(|v| v.as_str()).unwrap_or("");
    let port = command_data.get("port").and_then(|v| v.as_str()).unwrap_or("");
    let os_type = command_data.get("os_type").and_then(|v| v.as_str()).unwrap_or("");
    let arch_type = command_data.get("arch_type").and_then(|v| v.as_str()).unwrap_or("");
    let os_version = command_data.get("os_version").and_then(|v| v.as_str()).unwrap_or("");


    println!("IP or Name: {}", ip_or_name);
    println!("Port: {}", port);
    println!("OS Type: {}", os_type);
    println!("Arch Type: {}", arch_type);
    println!("OS Version: {}", os_version);
    println!("Caller ID: {}", caller_id);

    Ok(())
}
