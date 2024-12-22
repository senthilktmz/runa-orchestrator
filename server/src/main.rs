use std::future::Future;
use std::pin::Pin;
use actix_web::{web, App, HttpServer, Responder, HttpResponse};

struct Route {
    path: &'static str,
    handler: fn() -> Pin<Box<dyn Future<Output =HttpResponse >>>
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

fn boxed_health() -> Pin<Box<dyn Future<Output = HttpResponse >>> {
    Box::pin(health())
}

const ROUTES_LIST: &[Route] = &[
    Route{
        path: "/health",
        handler: boxed_health
    },
];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        let tmpApp = ROUTES_LIST.iter().fold(App::new(), |app, route| {
            app.route(route.path,  web::get().to(route.handler))
        });
        tmpApp
    }).bind("127.0.0.1:8080")?.run().await
}


