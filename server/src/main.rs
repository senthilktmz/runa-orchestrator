mod orchestrator;
mod orchestrator_routes;

use runautils::actix_server_util::{serve_requests, Route};
use orchestrator::generic_handlers::{boxed_get_req, boxed_post_handler};
use orchestrator::health_calls::boxed_health;
use orchestrator::ws_handle_task_request;
use orchestrator::task_agent;



#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = orchestrator_routes::routes();
    serve_requests(routes).await
}
