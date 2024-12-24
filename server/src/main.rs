use std::future::Future;
use std::pin::Pin;
use actix_web::{guard::Method, http::Method, web, App, HttpResponse, HttpServer, Responder};

struct Route {
    path: &'static str,
    handler: fn() -> Pin<Box<dyn Future<Output =HttpResponse >>>,
    request_type: Method,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

fn boxed_health() -> Pin<Box<dyn Future<Output = HttpResponse >>> {
    Box::pin(health())
}

async fn post_req(req_body: String) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "received": req_body
    }))
}

fn boxed_post_req() -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    println!("osososos");
    Box::pin(post_req("".to_string()))
}

const ROUTES_LIST: &[Route] = &[
    Route{
        path: "/health",
        request_type: Method::GET,
        handler: boxed_health
    },
    Route {
        path: "/post_req",
        request_type: Method::POST,
        handler: boxed_post_req
    }
];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    serve_requests().await
}

async fn serve_requests() -> std::io::Result<()> {
    println!("Starting server");
    HttpServer::new(|| {
        let tmp_app = ROUTES_LIST.iter().fold(App::new(), |app, route| {
            match route.request_type {
                Method::GET => app.route(route.path, web::get().to(route.handler)),
                Method::POST => app.route(route.path, web::post().to(|body:String| async move {
                    post_req(body).await
                })),
                _ => app,
            }
        });

        tmp_app
    }).bind("127.0.0.1:8080")?
    .run()
    .await
}

