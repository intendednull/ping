use std::{collections::HashMap, rc::Rc, str::FromStr};

use protocol::Peer;
use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::{dispatch, prelude::*};

use common::address::Address;

use crate::net::Client;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct SpaceAddress(pub Address);

impl FromStr for SpaceAddress {
    type Err = <Address as FromStr>::Err;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(Address::from_str(s)?))
    }
}

impl std::fmt::Display for SpaceAddress {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Action {
    /// Send a message to a space.
    SendMessage(SpaceAddress, Message),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Message {
    pub author: Peer,
    pub text: String,
}

#[derive(Debug, Default, Clone, PartialEq, Serialize, Deserialize)]
pub struct Space {
    messages: Vec<Message>,
}

impl Space {
    fn add_message(&mut self, mut message: Message, author: Peer) {
        message.author = author;
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
pub struct Spaces(HashMap<SpaceAddress, Rc<Space>>);

impl Spaces {
    pub fn handle_action(&mut self, action: Action, peer: protocol::Peer) {
        match action {
            Action::SendMessage(address, message) => {
                let space = self.space_mut(&address);
                space.add_message(message, peer);
            }
        }
    }

    pub fn get(&self, address: &SpaceAddress) -> Option<Rc<Space>> {
        self.0.get(address).cloned()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&SpaceAddress, &Rc<Space>)> {
        self.0.iter()
    }

    pub fn create_new_space(&mut self) -> SpaceAddress {
        let address = SpaceAddress(Address::new());
        // This will create the space if it doesn't exist.
        self.space_mut(&address);

        address
    }

    pub fn load_space(&mut self, address: &SpaceAddress, space: Space) {
        *self.space_mut(address) = space
    }

    fn space_mut(&mut self, address: &SpaceAddress) -> &mut Space {
        let space = self.0.entry(address.clone()).or_default();

        Rc::make_mut(space)
    }
}

#[hook]
pub fn use_space(address: &SpaceAddress) -> Rc<Space> {
    let client = use_store_value::<Client>();
    let space = use_selector_with_deps(
        move |spaces: &Spaces, address| match spaces.get(address) {
            Some(space) => space,
            None => {
                client.join_space(address).unwrap();
                // Add new space to spaces.
                let address = address.clone();
                Dispatch::<Spaces>::new()
                    .reduce(move |s| s.load_space(&address, Default::default()));
                // Return an empty space to start.
                Space::default().into()
            }
        },
        address.clone(),
    );

    space.as_ref().clone()
}

#[cfg(test)]
mod tests {
    use protocol::Identity;

    use super::*;

    #[test]
    fn space_add_message_uses_correct_author() {
        let i1 = Identity::new().as_peer();
        let i2 = Identity::new().as_peer();
        let mut space = Space::default();

        space.add_message(
            Message {
                author: i1,
                text: "".into(),
            },
            i2.clone(),
        );

        let message = &space.messages[0];

        assert_eq!(message.author, i2)
    }
}
