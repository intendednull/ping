use serde::{Deserialize, Serialize};

use protocol::identity::{Identity, Peer};

use crate::space::Space;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Presense {
    peer_id: Peer,
    alias: String,
}

impl Presense {
    pub fn apply(&self, space: &mut Space) {}
}
