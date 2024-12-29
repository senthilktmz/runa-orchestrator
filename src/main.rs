use clap::Parser;

mod orchestrator;

use runautils::actix_server_util::serve_requests;
use orchestrator::orchestrator_routes;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(long = "work-dir")]
    work_dir: String,
    #[arg(short, long)]
    port: String,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    println!("Serving on: {}", &args.work_dir);
    println!("Serving on: {}", &args.port);

    let work_dir = args.work_dir;
    let port = args.port;

    let routes = orchestrator_routes::routes();
    serve_requests(routes, work_dir, port).await
}

//
//