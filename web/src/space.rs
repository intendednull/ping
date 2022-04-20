use std::{collections::HashMap, rc::Rc, str::FromStr};

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yewdux::{dispatch, mrc::Mrc, prelude::*};

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
pub struct Spaces(HashMap<SpaceAddress, Rc<Space>>);

impl Spaces {
    pub fn handle_action(&mut self, action: Action) {
        match action {
            Action::Send(address, message) => {
                let space = self.space_mut(&address);
                space.send_message(message);
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
    let address = address.clone();
    let (spaces, dispatch) = use_store::<Spaces>();
    let client = use_store_value::<Client>();

    match spaces.get(&address) {
        Some(space) => space,
        None => {
            client.join_space(&address).unwrap();
            dispatch.reduce(move |s| s.load_space(&address, Default::default()));
            Space::default().into()
        }
    }
}
