use lunatic::net::TcpStream;
use serde::{Deserialize, Serialize};
use tungstenite::server;
use wactor::*;

use common::transport::Message;

use crate::node::{self, Node};

#[derive(Deserialize, Serialize)]
pub struct Input {
    pub stream: TcpStream,
    pub node: Bridge<Node>,
    pub id: u32,
}

pub struct Client;
impl Actor for Client {
    type Input = Input;
    type Output = ();

    fn create() -> Self {
        Self
    }

    fn handle(&mut self, Input { node, stream, id }: Self::Input, _link: &Link<Self>) {
        let mut ws = server::accept(stream).expect("Error creating WS");
        loop {
            let msg = ws.read_message();
            match msg {
                Ok(tungstenite::Message::Binary(msg)) => {
                    let msg =
                        bincode::deserialize::<Message>(&msg).expect("Error deserializing message");

                    log::info!("Received: {:?}", msg);

                    let response = node
                        .get(node::Input::Message { client_id: id, msg })
                        .expect("Node is dead");
                    match response {
                        node::Output::Message(msg) => {
                            log::info!("Sent: {:?}", msg);

                            let buf = bincode::serialize(&msg).expect("Error serializing response");
                            ws.write_message(buf.into())
                                .expect("Error sending response");
                        }
                        _ => {
                            log::info!("Unexpected response from node")
                        }
                    }
                }
                Err(_) => break,
                _ => {}
            }
        }
    }
}
