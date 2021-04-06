use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::channel::Channel;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Join room with given id.
    JoinChannel(String),
    /// Send a message to room with given id.
    Channel(Channel),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Channel(Channel),
}

pub fn pack<T: Serialize>(data: &T) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::serialize(data)?)
}

pub fn unpack<T: DeserializeOwned>(data: &Vec<u8>) -> anyhow::Result<T> {
    Ok(bincode::deserialize(data)?)
}
