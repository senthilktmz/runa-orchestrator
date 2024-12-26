use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{web, HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::time::Duration;

async fn websocket_handler_impl(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> Result<HttpResponse, actix_web::Error> {
    let start = ws::start(WebSocketActor, &req, stream);
    start
}

pub fn websocket_handler(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>> {
    Box::pin(websocket_handler_impl(req, stream))
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
