
use actix_web::{web, HttpResponse};
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::any::Any;
use crate::orchestrator::generic_handlers::{extract_payload, ServerContext};
use serde_json::{Value};
use crate::orchestrator::add_task_agent;

async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}


pub fn post_handler(
    body: web::Json<String>,
    path: &'static str,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {


    match extract_payload(body, path, server_context) {
        Ok((decrypted_payload, original_body)) => {

            handle_task_agent_request(decrypted_payload);

            Box::pin(async {
                HttpResponse::Ok().body(format!("{}", "{}"))
            })
        }
        Err(err) => {
            println!("Error in extract_payload: {}", err);
            Box::pin(async {
                HttpResponse::InternalServerError().body(format!("Error: {}", "payload format wrong"))
            })
        }
    }
}

fn handle_task_agent_request(payload :String) -> Result<(), String> {

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
            add_task_agent::process_add_task_agent(command_params)?;
        }
        _ => {
            println!("Unsupported command type: {}", command_type);
        }
    }

    Ok(())
}


