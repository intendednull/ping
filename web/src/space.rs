use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use common::address::Address;
use yewdux::{dispatch, prelude::Store};

use crate::net::Client;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceAddress(pub Address);

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Send a message to a space.
    Send(SpaceAddress, Message),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub text: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Space {
    messages: Vec<Message>,
}

impl Space {
    fn send_message(&mut self, message: Message) {
        self.messages.push(message);
    }

    pub fn messages(&self) -> impl Iterator<Item = &Message> {
        self.messages.iter()
    }
}

pub fn join_spaces() {
    let spaces = dispatch::get::<Spaces>();
    let client = dispatch::get::<Client>();

    for (address, _) in spaces.iter() {
        client.join_space(address).unwrap();
    }
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize, Store)]
#[store(storage = "local")]
pub struct Spaces(HashMap<SpaceAddress, Space>);

impl Spaces {
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Send(address, message) => {
                let space = self.space_mut(address);
                space.send_message(message);
            }
        }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SpaceAddress, &Space)> {
        self.0.iter()
    }

    pub fn create_new_space(&mut self) -> SpaceAddress {
        let address = SpaceAddress(Address::new());
        // This will create the space if it doesn't exist.
        self.space_mut(address.clone());

        address
    }

    fn space_mut(&mut self, address: SpaceAddress) -> &mut Space {
        self.0.entry(address).or_default()
    }
}
