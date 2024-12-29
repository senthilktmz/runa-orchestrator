use actix_web::{web, HttpResponse};
use std::future::Future;
use std::pin::Pin;
use runautils::actix_server_util::ServerContext;
use std::sync::Arc;


async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}

pub fn post_handler(
    body: web::Json<String>,
    path: &'static str,
    server_context: Arc<ServerContext>
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(post_req(body, path))
}

