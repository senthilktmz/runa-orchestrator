use actix_web::{web, HttpResponse};
use runautils::actix_server_util::ServerStateStore;
use serde_json::Value;
use std::any::Any;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use crate::orchestrator::add_task_agent::process_add_task_agent;
use crate::orchestrator::generic_handlers::{extract_payload};
use crate::orchestrator::run_bash_script::process_run_bash_script;

async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}

pub fn post_handler(
    body: web::Json<String>,
    path: &'static str,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    match extract_payload(body, path, server_context) {
        Ok((decrypted_payload, original_body)) => {
            match handle_task_agent_request(decrypted_payload, server_state_store) {
                Ok(response) =>  {
                    return response
                },
                Err(err) => Box::pin(async {
                        return HttpResponse::InternalServerError()
                            .body(format!("Error: {}", "invalid request type"))
                    })
            }
        }
        Err(err) => {
            println!("Error in extract_payload: {}", err);
            Box::pin(async {
                HttpResponse::InternalServerError()
                    .body(format!("Error: {}", "payload format wrong"))
            })
        }
    }
}

fn handle_task_agent_request(
    payload: String,
    server_state: Arc<Mutex<ServerStateStore>>,
) -> Result<Pin<Box<dyn Future<Output = HttpResponse>>>, String> {
    let parsed_json: Value = serde_json::from_str(payload.as_str()).map_err(|e| e.to_string())?;

    let command_params = parsed_json
        .get("request_params")
        .and_then(|v| v.get("command_params"))
        .ok_or("Missing command_params")?;

    let command_type = command_params
        .get("command_type")
        .and_then(|v| v.as_str())
        .ok_or("Missing command_type")?;

    match command_type {
        "add_task_agent" => {
            let response = process_add_task_agent(command_params, server_state)?;
            return Ok(response);
        },
        "run_bash_script" => {
            let response = process_run_bash_script(command_params, server_state)?;
            return Ok(response);
        }
        _ => {
            println!("Unsupported command type: {}", command_type);
        }
    }

    Err(format!("Unsupported command type: {}", command_params))
}

#[derive(Debug, Clone)]
pub struct TaskAgent {
    pub ip_or_name: String,
    pub caller_id: String,
    pub port: String,
    pub os_type: String,
    pub arch_type: String,
    pub os_version: String,
}
