mod orchestrator;
mod orchestrator_routes;

use runautils::actix_server_util::{serve_requests};

#[cfg( feature =  "server_type_orchestrator")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let routes = orchestrator_routes::routes();
    serve_requests(routes).await
}

#[cfg(feature = "server_type_task_agent")]
fn main() {
    println!("Worker Server");
}

#[cfg(not(any(feature = "server_type_orchestrator", feature = "server_type_task_agent")))]
fn main() {
    println!("Unknown server type");
}

//
//