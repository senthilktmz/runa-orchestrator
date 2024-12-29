
use actix_web::{web, HttpResponse};
//use runautils::actix_server_util::ServerContext;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;
use std::any::Any;
use runautils::cipher_item;
use crate::orchestrator::generic_handlers::{extract_payload, ServerContext};


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
