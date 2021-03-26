use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wactor::*;

use common::transport as t;

use crate::client::{Listener, Responder};

#[derive(Deserialize, Serialize)]
pub enum Input {
    Message {
        client_id: u32,
        msg: t::Message,
    },
    RegisterResponder {
        client_id: u32,
        responder: Bridge<Responder>,
    },
}

#[derive(Deserialize, Serialize)]
pub enum Output {
    Message(t::Message),
}

pub struct Node {
    clients: HashMap<u32, Bridge<Responder>>,
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
            Input::RegisterResponder {
                client_id,
                responder: listener,
            } => {
                self.clients.insert(client_id, listener);
            }
            Input::Message { client_id, msg } => {
                let response = match msg {
                    t::Message::Ping => t::Message::Pong,
                    t::Message::Pong => t::Message::Ping,
                };

                self.clients
                    .get(&client_id)
                    .expect("Unknown client")
                    .send(response)
                    .ok();
            }
        }
    }
}

impl From<t::Message> for Output {
    fn from(val: t::Message) -> Self {
        Output::Message(val)
    }
}
