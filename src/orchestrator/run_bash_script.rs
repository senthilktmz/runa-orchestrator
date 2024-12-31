use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use actix_web::HttpResponse;
use runautils::actix_server_util::ServerStateStore;
use serde_json::Value;

pub fn process_run_bash_script(
    command_params: &Value,
    server_state: Arc<Mutex<ServerStateStore>>,
) -> Result<Pin<Box<dyn Future<Output = HttpResponse>>>, String> {
    println!("{}", "process_run_bash_script");
    Ok(Box::pin(async { HttpResponse::Ok().body(format!("{}", "val")) }))
}