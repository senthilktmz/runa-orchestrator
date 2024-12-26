
use std::process;
use clap::Parser;

mod orchestrator;
mod orchestrator_routes;

use runautils::actix_server_util::{serve_requests};



#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Name of the person to greet
    #[arg(short, long)]
    work_dir: String,
    #[arg(short, long)]
    port: String,
}


#[cfg( feature =  "server_type_orchestrator")]
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    // for _ in 0..args.count {
    //     println!("Hello {}!", args.name);
    // }

    std::process::exit(1);

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