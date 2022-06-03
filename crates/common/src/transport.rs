use std::rc::Rc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::address::Address;

/// Message sent to a node for routing.
#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub enum Input {
    Send(Address, Rc<Vec<u8>>),
    Join(Address),
}

#[derive(PartialEq, Eq, Serialize, Deserialize, Debug, Clone)]
pub struct Output {
    pub payload: Rc<Vec<u8>>,
    pub address: Address,
}

pub fn pack<T: Serialize>(data: &T) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::serialize(data)?)
}

pub fn unpack<T: DeserializeOwned>(data: &[u8]) -> anyhow::Result<T> {
    Ok(bincode::deserialize(data)?)
}
