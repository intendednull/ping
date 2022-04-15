use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::address::Address;

pub struct Message;

pub struct Spaces {
    spaces: HashMap<Address, Space>,
}

impl Spaces {
    fn new_space(&mut self) -> Address {
        let space = Space::new();
        self.add_space(space)
    }

    fn add_space(&mut self, space: Space) -> Address {
        let address = space.address.clone();

        self.spaces.insert(address.clone(), space);

        address
    }
}

pub struct Space {
    address: Address,
    participants: Vec<Address>,
    messages: Vec<Message>,
}

impl Space {
    fn new() -> Self {
        Self {
            address: Address::new(),
            participants: Default::default(),
            messages: Default::default(),
        }
    }
}
