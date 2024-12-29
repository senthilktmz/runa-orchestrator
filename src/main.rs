use std::any::Any;
use std::sync::Arc;
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

#[derive(Clone)]
pub struct ServerExecDependencies<'a> {
    pub http_request_decrypt_key: &'a[u8; 32],
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let args = Args::parse();

    let dependencies = match inject_dependencies() {
        Ok(dependencies) => dependencies,
        Err(e) => {
            println!("Unable to resolve dependencies");
            std::process::exit(1);
        }
    };

    let work_dir = args.work_dir;
    let port = args.port;

    let routes = orchestrator_routes::routes();
    serve_requests(routes, work_dir, port, dependencies).await
}

fn get_http_request_decrypt_key() -> &'static [u8; 32] {
    let test_key = &b"0123456789abcdef0123456789abcdef";
    return test_key
}

fn inject_dependencies() -> Result<Arc<Box<dyn Any + Send + Sync>>, String> {
    let dependencies: Arc<Box<dyn Any + Send + Sync>> =
        Arc::new(Box::new(ServerExecDependencies{
            http_request_decrypt_key: get_http_request_decrypt_key(),
        }));
    Ok(dependencies)
}

//
//