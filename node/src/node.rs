use serde::{Deserialize, Serialize};
use slab::Slab;
use wactor::*;

use common::transport as t;

use crate::client::Responder;

#[derive(Deserialize, Serialize)]
pub enum Input {
    Message { client_id: usize, msg: t::Message },
    RegisterClient(Bridge<Responder>),
}

#[derive(Deserialize, Serialize)]
pub enum Output {
    /// Receive a message from client.
    Message(t::Message),
    /// Assign an internal id the client. This is **not** a unique identifier for individual
    /// users, and will be reused when connection closes.
    ClientId(usize),
}

pub struct Node {
    /// Tracking connected clients.
    clients: Slab<Bridge<Responder>>,
}

impl Actor for Node {
    type Input = Input;
    type Output = Output;

    fn create() -> Self {
        Self {
            clients: Default::default(),
        }
    }

    fn handle(&mut self, msg: Self::Input, link: &Link<Self>) {
        match msg {
            Input::RegisterClient(responder) => {
                let id = self.clients.insert(responder);
                link.respond(Output::ClientId(id)).ok();
            }
            Input::Message { client_id, msg } => {
                let response = match msg {
                    t::Message::Ping => t::Message::Pong,
                    t::Message::Pong => t::Message::Ping,
                };

                if let Some(client) = self.clients.get(client_id) {
                    client.send(response).ok();
                }
            }
        }
    }
}

impl From<t::Message> for Output {
    fn from(val: t::Message) -> Self {
        Output::Message(val)
    }
}
