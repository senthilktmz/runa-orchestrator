use actix_web::{web, HttpResponse};
use std::future::Future;
use std::pin::Pin;


async fn post_req(body: web::Json<String>, path: &'static str) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "received": *body, "path": path }))
}

pub fn post_handler(
    body: web::Json<String>,
    path: &'static str,
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(post_req(body, path))
}

