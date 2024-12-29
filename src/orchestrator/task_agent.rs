use actix_web::{web, HttpResponse};
use std::future::Future;
use std::pin::Pin;
use runautils::actix_server_util::ServerContext;
use std::sync::Arc;

use runautils::cipher_item;

async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}

pub fn post_handler(
    body: web::Json<String>,
    path: &'static str,
    server_context: Arc<ServerContext>
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {


    let result = cipher_item::get_decrypted_payload(body.as_str().to_string(),
                                              b"0123456789abcdef0123456789abcdef");

    match result {
        Ok(decrypted) => {
            println!("Decrypted Payload: {}", decrypted);
            //Box::pin(async { HttpResponse::Ok().body("Decrypted successfully") })
        }
        Err(err) => {
            println!("Decryption failed: {}", err);
            //Box::pin(async { HttpResponse::InternalServerError().body("Decryption failed") })
        }
    }

    Box::pin(post_req(body, path))
}