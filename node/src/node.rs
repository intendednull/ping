use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use wactor::*;

use common::transport as t;

use crate::client::Client;

#[derive(Deserialize, Serialize)]
pub enum Input {
    Message { client_id: u32, msg: t::Message },
}

#[derive(Deserialize, Serialize)]
pub enum Output {
    Message(t::Message),
}

pub struct Node {
    // clients: HashMap<u32, Bridge<Client>>,
}

impl Actor for Node {
    type Input = Input;
    type Output = Output;

    fn create() -> Self {
        Self {
            // clients: Default::default(),
        }
    }

    fn handle(&mut self, msg: Self::Input, link: &Link<Self>) {
        match msg {
            Input::Message { client_id, msg } => {
                let response = match msg {
                    t::Message::Ping => t::Message::Pong,
                    t::Message::Pong => t::Message::Ping,
                };

                link.respond(response).ok();
            }
        }
    }
}

impl From<t::Message> for Output {
    fn from(val: t::Message) -> Self {
        Output::Message(val)
    }
}
