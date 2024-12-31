use std::any::Any;
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use runautils::actix_server_util::ServerStateStore;

pub fn websocket_handler2(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>> {
    Box::pin(async move {

        println!("WebSocket handler invoked with server context and state store");

        ws::start(WebSocketActor, &req, stream)
    })
}

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
