use serde_json::{self, Value};
use std::any::Any;
use actix::{Actor, ActorContext, AsyncContext, StreamHandler};
use actix_web::{HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use runautils::actix_server_util::ServerStateStore;
use crate::orchestrator::generic_handlers::extract_payload_from_string;
use tokio_tungstenite::{connect_async, tungstenite::Message};
use futures_util::{SinkExt, StreamExt};
use url::Url;
use actix::ActorFutureExt;

pub fn websocket_handler2(
    req: actix_web::HttpRequest,
    stream: actix_web::web::Payload,
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
) -> Pin<Box<dyn Future<Output = Result<HttpResponse, actix_web::Error>>>> {
    Box::pin(async move {
        println!("WebSocket handler invoked with server context and state store");
        ws::start(WebSocketActor {
            server_context,
            server_state_store,
        }, &req, stream)
    })
}

struct WebSocketActor {
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        println!("WebSocket connection started");
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        println!("WebSocket connection stopped");
    }
}

/// Function to process the JSON message
fn process_json_message(
    json_value: &Value,
    server_context: &Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: &Arc<Mutex<ServerStateStore>>,
) -> Result<String, String> {

    println!("--------------------------------------{}", "");
    println!("{:#?}", json_value);
    println!("--------------------------------------{}", "");
    Ok(format!("Processed task: {}", "ok"))
}

async fn forward_to_task_executor(message: &str) -> Result<String, String> {
    let url = Url::parse("ws://127.0.0.1:9292/exec_task_set")
        .map_err(|e| format!("URL parse error: {}", e))?;

    let (ws_stream, _) = connect_async(url)
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    let (mut write, mut read) = ws_stream.split();

    // Forward the original message
    write.send(Message::Text(message.to_string()))
        .await
        .map_err(|e| format!("Send error: {}", e))?;

    // Keep reading messages until connection closes
    let mut all_responses = Vec::new();
    while let Some(msg) = read.next().await {
        match msg {
            Ok(Message::Text(response)) => {
                println!("{}", response);
                all_responses.push(response);
            },
            Ok(Message::Close(_)) => {
                break;
            },
            Ok(_) => continue, // Skip other message types
            Err(e) => return Err(format!("Receive error: {}", e))
        }
    }

    if all_responses.is_empty() {
        Err("No responses received".to_string())
    } else {
        Ok(all_responses.join("\n"))
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                let text_clone = text.clone();
                let fut = async move {
                    match forward_to_task_executor(&text_clone).await {
                        Ok(response) => response,
                        Err(e) => format!("Task executor error: {}", e)
                    }
                };

                ctx.spawn(
                    actix::fut::wrap_future(fut)
                        .map(|response, _actor: &mut WebSocketActor, ctx: &mut ws::WebsocketContext<WebSocketActor>| {
                            ctx.text(response);
                        })
                );
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Received binary message: {:?}", bin);
                ctx.binary(bin);
            }
            Ok(ws::Message::Close(_)) => {
                println!("Client closed the connection");
                ctx.stop();
            }
            Err(err) => {
                println!("WebSocket error: {}", err);
                ctx.stop();
            }
            _ => (),
        }
    }
}