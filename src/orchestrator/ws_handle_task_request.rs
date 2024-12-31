use serde_json::{self, Value};
use std::any::Any;
use actix::{Actor, ActorContext, StreamHandler};
use actix_web::{HttpResponse};
use actix_web_actors::ws;
use std::future::Future;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use runautils::actix_server_util::ServerStateStore;
use crate::orchestrator::generic_handlers::extract_payload_from_string;

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

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WebSocketActor {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {

                let json_request = text.to_string();
                match extract_payload_from_string(text.to_string(), "N/A", &self.server_context) {
                    Ok(json_payload) => {
                        println!("Received json payload: {:?}", json_payload);
                        // Parse the JSON text
                        let (json_string, _) = json_payload;
                        match serde_json::from_str::<Value>(json_string.as_str()) {
                            Ok(json_value) => {
                                println!("Parsed JSON: {:?}", json_value);

                                // Process the JSON message
                                let response = process_json_message(&json_value, &self.server_context, &self.server_state_store);

                                // Send the response back to the client
                                match response {
                                    Ok(response_text) => ctx.text(response_text),
                                    Err(err) => ctx.text(format!("Error: {}", err)),
                                }
                            }
                            Err(err) => {
                                println!("Failed to parse JSON: {}", err);
                                ctx.text(format!("Invalid JSON: {}", err));
                            }
                        }
                    },
                    Err(err) => {
                        println!("Received error payload: {:?}", err);
                    }
                }
            }
            Ok(ws::Message::Binary(bin)) => {
                println!("Received binary message: {:?}", bin);
                ctx.binary(bin); // Echo the binary message back
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
