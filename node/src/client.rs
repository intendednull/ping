use lunatic::net::TcpStream;
use serde::{Deserialize, Serialize};
use tungstenite::{protocol::Role, server, WebSocket};
use wactor::*;

use common::transport::{self as t, Request, Response};

use crate::node::{self, Node};

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub stream: TcpStream,
    pub node: Bridge<Node>,
}

/// Actor for receiving messages from client.
pub struct Listener;
impl Actor for Listener {
    type Input = Config;
    type Output = ();

    fn create() -> Self {
        Self
    }

    fn handle(&mut self, Config { node, stream }: Self::Input, _link: &Link<Self>) {
        // Accept a ws connection.
        let mut ws = server::accept(stream.clone()).expect("Error creating WS");
        // Spawn a responder to handle responsed from node. Needed because this listener will
        // always be blocking to read messages.
        let responder = wactor::spawn::<Responder>();
        responder
            .send(ResponderInput::Init(stream))
            .expect("responder");

        // Register this client's responder with node.
        let id = match node
            .get(node::Input::RegisterClient(responder))
            .expect("register responder")
        {
            node::Output::ClientId(id) => id,
            _ => panic!("Bad response from node"),
        };

        loop {
            match ws.read_message() {
                Ok(tungstenite::Message::Binary(msg)) => {
                    if let Ok(msg) = t::unpack(&msg) {
                        log::info!("Received: {:?}", msg);

                        node.send(node::Input::Request { client_id: id, msg }).ok();
                    }
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
    Response(Response),
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
            ResponderInput::Response(msg) => {
                log::info!("Sending {:?}", msg);
                if let Ok(data) = t::pack(&msg) {
                    self.ws
                        .as_mut()
                        .expect("not initialized")
                        .write_message(data.into())
                        .ok();
                }
            }
        }
    }
}

impl From<Response> for ResponderInput {
    fn from(val: Response) -> Self {
        ResponderInput::Response(val)
    }
}
