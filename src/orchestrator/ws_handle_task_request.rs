
use actix::*;
use tokio::sync::mpsc;
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

//struct WebSocketActor;

struct WebSocketActor {
    server_context: Arc<Box<dyn Any + Send + Sync>>,
    server_state_store: Arc<Mutex<ServerStateStore>>,
}

impl Actor for WebSocketActor {
    type Context = ws::WebsocketContext<Self>;
}

// Custom Actix message to send responses to the client
struct SendToClient(String);

impl actix::Message for SendToClient {
    type Result = ();
}

impl actix::Handler<SendToClient> for WebSocketActor {
    type Result = ();

    fn handle(&mut self, msg: SendToClient, ctx: &mut Self::Context) {
        ctx.text(msg.0); // Send the message to the WebSocket client
    }
}

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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Create a channel to pass responses back to the actor
                let (sender, mut receiver) = mpsc::channel(100);

                // Clone the actor's address to send messages back
                let addr = ctx.address();

                // Spawn an async task for the processing logic
                ctx.spawn(
                    actix::fut::wrap_future(async move {
                        if let Err(err) = forward_to_task_executor(&text, sender).await {
                            eprintln!("Error in forward_to_task_executor: {}", err);
                        }
                    }),
                );

                // Spawn another task to receive responses and send them via the actor
                ctx.spawn(
                    actix::fut::wrap_future(async move {
                        while let Some(response) = receiver.recv().await {
                            addr.do_send(SendToClient(response));
                        }
                    }),
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

// The asynchronous function for task execution
async fn forward_to_task_executor(message: &str, sender: mpsc::Sender<String>) -> Result<(), String> {
    let url = url::Url::parse("ws://127.0.0.1:9292/exec_task_set")
        .map_err(|e| format!("URL parse error: {}", e))?;

    let (ws_stream, _) = tokio_tungstenite::connect_async(url)
        .await
        .map_err(|e| format!("Connection error: {}", e))?;

    let (mut write, mut read) = ws_stream.split();

    // Forward the original message
    write
        .send(tokio_tungstenite::tungstenite::protocol::Message::Text(message.to_string()))
        .await
        .map_err(|e| format!("Send error: {}", e))?;

    // Read responses and send them back via the channel
    while let Some(msg) = read.next().await {
        match msg {
            Ok(tokio_tungstenite::tungstenite::protocol::Message::Text(response)) => {
                if sender.send(response).await.is_err() {
                    eprintln!("Actor has dropped the channel");
                    break;
                }
            }
            Ok(tokio_tungstenite::tungstenite::protocol::Message::Close(_)) => {
                break;
            }
            Ok(_) => continue, // Skip other message types
            Err(e) => return Err(format!("Receive error: {}", e)),
        }
    }

    Ok(())
}
