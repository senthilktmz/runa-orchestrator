use runautils::actix_server_util::Route;
use crate::orchestrator::generic_handlers::{boxed_get_req, boxed_post_handler};
use crate::orchestrator::health_calls::boxed_health;
use crate::orchestrator::{task_agent, ws_handle_task_request};

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
    Route {
        path: "/task_agent",
        get_handler: None,
        post_handler: Some(task_agent::post_handler),
        websocket_handler: None,
    }
];

pub fn routes() -> Vec<Route> {
    return ROUTES_LIST.to_vec();
}