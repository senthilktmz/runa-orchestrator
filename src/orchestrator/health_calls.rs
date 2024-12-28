use actix_web::HttpResponse;
use std::future::Future;
use std::pin::Pin;

pub async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

pub fn boxed_health() -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(health())
}
