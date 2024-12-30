use actix_web::{web, HttpResponse};
//use runautils::actix_server_util::ServerContext;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::any::Any;
use std::collections::HashMap;
use runautils::actix_server_util::ServerStateStore;
use uuid::Uuid;
use runautils::cipher_item;

pub async fn get_req() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "generic http get handler" }))
}

pub fn boxed_get_req() -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(get_req())
}

pub async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}

pub fn boxed_post_handler(
    body: web::Json<String>,
    path: &'static str,
    server_context:  Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(post_req(body, path))
}

#[derive(Clone)]
#[derive(Debug)]
pub struct ServerContext<'a> {
    pub http_request_decrypt_key: &'a [u8; 32],
    pub state_storage_map : HashMap<String, Arc<Box<dyn Any + Send + Sync>>>,
    pub server_execution_instance_uuid: String,
}

pub fn extract_payload(
    body: web::Json<String>,
    path: &'static str,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
) -> Result<(String, web::Json<String>), String> {

    if let Some(server_context) = server_context.downcast_ref::<ServerContext>() {

        let key = server_context.http_request_decrypt_key;
        let result = cipher_item::get_decrypted_payload(
            body.as_str().to_string(),
            server_context.http_request_decrypt_key,
        );

        match result {
            Ok(decrypted) => {
                return Ok((decrypted, body));
            }
            Err(err) => {
                return Err(String::from("8c9a1eb4-a119-450c-967d-f53b0826e5e1"))
            }
        }
    } else {
        println!("Failed to downcast to ServerContext.");
        Err(String::from("71b3a699-4166-426d-aa24-eb59660a935e"))
    }
}

//
//