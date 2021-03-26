use lunatic::net::TcpStream;
use serde::{Deserialize, Serialize};
use tungstenite::{protocol::Role, server, WebSocket};
use wactor::*;

use common::transport::Message;

use crate::node::{self, Node};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub stream: TcpStream,
    pub node: Bridge<Node>,
    pub id: u32,
}

/// Actor for receiving messages from client.
pub struct Listener;
impl Actor for Listener {
    type Input = Config;
    type Output = ();

    fn create() -> Self {
        Self
    }

    fn handle(&mut self, Config { node, stream, id }: Self::Input, _link: &Link<Self>) {
        // Create a new ws server.
        let mut ws = server::accept(stream.clone()).expect("Error creating WS");
        // Spawn a responder to handle responsed from node. Needed because this listener will
        // always be blocking to read messages.
        let responder = wactor::spawn::<Responder>();
        responder
            .send(ResponderInput::Init(stream))
            .expect("responder");
        // Register this client's responder with node.
        node.send(node::Input::RegisterResponder {
            client_id: id,
            responder,
        })
        .expect("register responder");

        loop {
            let msg = ws.read_message();
            match msg {
                Ok(tungstenite::Message::Binary(msg)) => {
                    let msg =
                        bincode::deserialize::<Message>(&msg).expect("Error deserializing message");

                    log::info!("Received: {:?}", msg);

                    node.send(node::Input::Message { client_id: id, msg }).ok();
                }
                // TODO: If stream is closed, shutdown responder and deregister from node.
                Err(_) => break,
                _ => {}
            }
        }
    }
}

#[derive(Deserialize, Serialize)]
pub enum ResponderInput {
    Init(TcpStream),
    Send(Message),
}

/// Actor for sending events to client.
pub struct Responder {
    ws: Option<WebSocket<TcpStream>>,
}

impl Actor for Responder {
    type Input = ResponderInput;
    type Output = ();

    fn create() -> Self {
        Self {
            ws: Default::default(),
        }
    }

    fn handle(&mut self, msg: Self::Input, _link: &Link<Self>) {
        match msg {
            ResponderInput::Init(stream) => {
                // Connect to stream, but don't perform a handshake.
                self.ws = Some(WebSocket::from_raw_socket(stream, Role::Server, None));
            }
            ResponderInput::Send(msg) => {
                let buf = bincode::serialize(&msg).expect("Error serializing response");
                self.ws
                    .as_mut()
                    .expect("not initialized")
                    .write_message(buf.into())
                    .ok();
            }
        }
    }
}

impl From<Message> for ResponderInput {
    fn from(val: Message) -> Self {
        ResponderInput::Send(val)
    }
}
