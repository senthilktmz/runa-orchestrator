use clap::Parser;
use std::any::Any;
use std::sync::Arc;

mod orchestrator;

use orchestrator::{generic_handlers::ServerContext, orchestrator_routes};
use runautils::actix_server_util::serve_requests;
use uuid::Uuid;

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

    let server_context = match get_server_context() {
        Ok(context) => context,
        Err(e) => {
            println!("Unable to resolve dependencies");
            std::process::exit(1);
        }
    };

    let work_dir = args.work_dir;
    let port = args.port;

    let routes = orchestrator_routes::routes();
    serve_requests(routes, work_dir, port, server_context).await
}

fn get_http_request_decrypt_key() -> &'static [u8; 32] {
    let test_key = &b"0123456789abcdef0123456789abcdef";
    return test_key;
}

fn get_server_context() -> Result<Arc<Box<dyn Any + Send + Sync>>, String> {
    let server_context: Arc<Box<dyn Any + Send + Sync>> = Arc::new(Box::new(ServerContext {
        http_request_decrypt_key: get_http_request_decrypt_key(),
        state_storage_map: Default::default(),
        server_execution_instance_uuid: generate_uuid(),
    }));
    Ok(server_context)
}

fn generate_uuid() -> String {
    Uuid::new_v4().to_string()
}
//
//
