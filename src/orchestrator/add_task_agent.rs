use crate::orchestrator::task_agent::TaskAgent;
use runautils::actix_server_util::ServerStateStore;
use serde_json::Value;
use std::sync::{Arc, Mutex};

pub fn process_add_task_agent(
    command_params: &Value,
    server_state: Arc<Mutex<ServerStateStore>>,
) -> Result<(), String> {
    let command_data = command_params
        .get("command_data")
        .ok_or("Missing command_data")?;

    let ip_or_name = command_data
        .get("ip_or_name")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let caller_id = command_data
        .get("caller_id")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let port = command_data
        .get("port")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let os_type = command_data
        .get("os_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let arch_type = command_data
        .get("arch_type")
        .and_then(|v| v.as_str())
        .unwrap_or("");
    let os_version = command_data
        .get("os_version")
        .and_then(|v| v.as_str())
        .unwrap_or("");

    println!("IP or Name: {}", ip_or_name);
    println!("Port: {}", port);
    println!("OS Type: {}", os_type);
    println!("Arch Type: {}", arch_type);
    println!("OS Version: {}", os_version);
    println!("Caller ID: {}", caller_id);

    // Step 1: Acquire the lock on ServerStateStore
    let hashmap_lock = server_state
        .lock()
        .expect("Failed to acquire lock on ServerStateStore");
    println!("Acquired lock on ServerStateStore");

    let mut hashmap = hashmap_lock
        .state
        .lock()
        .expect("Failed to acquire lock on hashmap lock");

    println!("Current server state:");
    if hashmap.is_empty() {
        println!("  HashMap is empty");
    } else {
        for (key, value) in hashmap.iter() {
            println!("Key: {}", key);
            if let Some(task_agent) = value.as_ref().as_ref().downcast_ref::<TaskAgent>() {
                println!("  Agent details: {:?}", task_agent);
            } else {
                println!("  Value couldn't be downcasted to TaskAgent");
            }
        }
    }

    let task_agent = TaskAgent {
        ip_or_name: ip_or_name.to_string(),
        caller_id: caller_id.to_string(),
        port: port.to_string(),
        os_type: os_type.to_string(),
        arch_type: arch_type.to_string(),
        os_version: os_version.to_string(),
    };

    hashmap.insert(ip_or_name.to_string(), Arc::new(Box::new(task_agent)));

    Ok(())
}
