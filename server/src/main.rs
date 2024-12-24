use std::future::Future;
use std::pin::Pin;
use std::time::Duration;
use actix_web::{http::Method, web, App, HttpResponse, HttpServer};
use actix_web_actors::ws;
use actix::{Actor, StreamHandler};
use actix::ActorContext;
use actix::AsyncContext;

#[derive(Clone)]
struct Route {
    path: &'static str,
    handler: fn() -> Pin<Box<dyn Future<Output = HttpResponse>>>,
    request_type: Method,
}

async fn health() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({ "status": "healthy" }))
}

fn boxed_health() -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(health())
}

async fn post_req(req_body: String) -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "received": req_body
    }))
}

fn boxed_post_req() -> Pin<Box<dyn Future<Output = HttpResponse>>> {
    Box::pin(post_req("".to_string()))
}

const ROUTES_LIST: &[Route] = &[
    Route {
        path: "/health",
        request_type: Method::GET,
        handler: boxed_health,
    },
    Route {
        path: "/post_req",
        request_type: Method::POST,
        handler: boxed_post_req,
    },
];

/// WebSocket actor
struct WebSocketActor;

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        let mut elapsed = 0;
        ctx.run_interval(Duration::from_secs(1), move |_, ctx| {
            elapsed += 1;
            ctx.text(format!("{} second(s) elapsed", elapsed));
            if elapsed >= 10 {
                ctx.stop();
            }
        });
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Close(_)) = msg {
            ctx.stop();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = ROUTES_LIST.to_vec();

    // Wrap ws_handler to match the expected type
    let wrapped_ws_handler = |req, stream| {
        Box::pin(ws_handler(req, stream))
            as Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>>
    };

    serve_requests(routes, wrapped_ws_handler).await
}

async fn serve_requests(
    routes_list: Vec<Route>,
    websocket_handler: fn(
        actix_web::HttpRequest,
        actix_web::web::Payload,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>>,
) -> std::io::Result<()> {

    println!("Starting server");

    HttpServer::new(move || {
        let app = routes_list.iter().fold(App::new(), |app, route| match route.request_type {
            Method::GET => app.route(route.path, web::get().to(route.handler)),
            Method::POST => app.route(route.path, web::post().to(|body: String| async move {
                post_req(body).await
            })),
            _ => app,
        });

        // Add WebSocket route using the wrapped handler
        app.route("/ws", web::get().to(websocket_handler))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}

/// WebSocket handler function
async fn ws_handler(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    ws::start(WebSocketActor, &req, stream)
}
