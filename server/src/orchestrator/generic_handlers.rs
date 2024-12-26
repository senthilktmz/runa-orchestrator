use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

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
) -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(post_req(body, path))
}

