use std::future::Future;
use std::pin::Pin;
use actix_web::{guard::Method, http::Method, web, App, HttpResponse, HttpServer, Responder};

#[derive(Clone)]
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
    let routes = ROUTES_LIST.to_vec().clone();
    serve_requests(routes).await
}

async fn serve_requests(routes_list: Vec<Route>) -> std::io::Result<()> {
    println!("Starting server");

    HttpServer::new(move || {
        routes_list.iter().fold(App::new(), |app, route| match route.request_type {
            Method::GET => app.route(route.path, web::get().to(route.handler)),
            Method::POST => app.route(route.path, web::post().to(|body: String| async move {
                post_req(body).await
            })),
            _ => app,
        })
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

