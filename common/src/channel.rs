use serde::{Deserialize, Serialize};
use sha3::{digest, Sha3_256};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ChannelMsg {
    /// Unique identifier of this channel.
    pub id: String,
    /// Action to apply.
    pub action: Action,
    /// Hash of the block the message should be applied to.
    pub hash: digest::Output<Sha3_256>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum Action {
    /// Number of participants in this channel
    Participants(usize),
    Ping,
    Pong,
}
