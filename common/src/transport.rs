use std::rc::Rc;

use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::address::Address;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    Send(Address, Rc<Vec<u8>>),
    Join(Address),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Response(pub Rc<Vec<u8>>);

pub fn pack<T: Serialize>(data: &T) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::serialize(data)?)
}

pub fn unpack<T: DeserializeOwned>(data: &Vec<u8>) -> anyhow::Result<T> {
    Ok(bincode::deserialize(data)?)
}
