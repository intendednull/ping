use serde::{de::DeserializeOwned, Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Request {
    /// Join room with given id.
    JoinRoom(String),
    /// Send a message to room with given id.
    Group(Group),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Response {
    Group(Group),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Group {
    pub id: String,
    pub msg: GroupMsg,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum GroupMsg {
    Ping,
    Pong,
}

pub fn pack<T: Serialize>(data: &T) -> anyhow::Result<Vec<u8>> {
    Ok(bincode::serialize(data)?)
}

pub fn unpack<T: DeserializeOwned>(data: &Vec<u8>) -> anyhow::Result<T> {
    Ok(bincode::deserialize(data)?)
}
