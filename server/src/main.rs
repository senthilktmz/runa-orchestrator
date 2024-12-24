
mod health_calls;
mod generic_handlers;

use runautils::actix_server_util::{Route, serve_requests};
//use actix::{StreamHandler};
//use actix_web::{ HttpResponse};
//use actix_web_actors::ws;


use crate::health_calls::{boxed_health};
use crate::generic_handlers::{boxed_get_req, boxed_post_handler, boxed_websocket_handler};

const ROUTES_LIST: &[Route] = &[
    Route {
        path: "/health",
        get_handler: Some(boxed_health),
        post_handler: None,
        websocket_handler: None,
    },
    Route {
    path: "/get_req",
    get_handler: Some(boxed_get_req),
    post_handler: None,
    websocket_handler: None,
},
    Route {
        path: "/post_req",
        get_handler: None,
        post_handler: Some(boxed_post_handler),
        websocket_handler: None,
    },
    Route {
        path: "/ws",
        get_handler: None,
        post_handler: None,
        websocket_handler: Some(boxed_websocket_handler),
    },
];

/// WebSocket actor



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = ROUTES_LIST.to_vec();
    serve_requests(routes).await
}
