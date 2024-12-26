mod orchestrator;

use runautils::actix_server_util::{serve_requests, Route};
use orchestrator::generic_handlers::{boxed_get_req, boxed_post_handler};
use orchestrator::health_calls::boxed_health;
use orchestrator::ws_handle_task_request;

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
        path: "/exec_task_set",
        get_handler: None,
        post_handler: None,
        websocket_handler: Some(ws_handle_task_request::websocket_handler),
    },

];

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = ROUTES_LIST.to_vec();
    serve_requests(routes).await
}
