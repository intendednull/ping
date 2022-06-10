use std::collections::HashMap;

use gloo::timers::callback::{Interval};
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Duration, Utc};
use protocol::identity::PeerId;

use yewdux::{
    dispatch,
    prelude::{Dispatch, Store},
};

use crate::{
    net::Client,
    space::{Space, Universe},
};

pub static PRESENSE_INTERVAL: u32 = 3;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Msg {
    Presense(PeerPresense),
}

impl Msg {
    pub fn apply(self, space: &mut Space, peer_id: &PeerId) {
        match self {
            Msg::Presense(mut presense) => {
                presense.peer_id = peer_id.clone();
                presense.apply(space);
            }
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PeerPresense {
    pub peer_id: PeerId,
    pub alias: String,
    pub last_updated: DateTime<Utc>,
    pub is_typing: bool,
}

impl PeerPresense {
    fn new(peer_id: PeerId) -> Self {
        Self {
            peer_id,
            alias: "anon".to_string(),
            last_updated: Utc::now(),
            is_typing: false,
        }
    }

    pub fn apply(&self, space: &mut Space) {
        let mut presense = self.clone();
        presense.last_updated = Utc::now();

        space.presense.peers.insert(self.peer_id.clone(), presense);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presense {
    pub peers: HashMap<PeerId, PeerPresense>,
    pub local: PeerPresense,
}

impl Default for Presense {
    fn default() -> Self {
        Self {
            peers: Default::default(),
            local: PeerPresense::new(dispatch::get::<Client>().peer.clone()),
        }
    }
}

pub fn init() {
    Interval::new(PRESENSE_INTERVAL * 1000, move || {
        let client = dispatch::get::<Client>();
        let universe = dispatch::get::<Universe>();
        for (address, space) in universe.iter() {
            // Send presense data to space.
            client
                .action(
                    address,
                    crate::space::Action::Presense(Msg::Presense(space.presense.local.clone())),
                )
                .ok();
            // Get all presense data that has expired.
            let expired = space
                .presense
                .peers
                .values()
                .filter(|x| {
                    Utc::now() - x.last_updated > Duration::seconds((PRESENSE_INTERVAL + 1) as _)
                })
                .map(|x| x.peer_id.clone());
            // Remove stale presense data.
            Dispatch::<Universe>::new().reduce_mut(|universe| {
                for peer in expired {
                    universe.space_mut(address).presense.peers.remove(&peer);
                }
            });
        }
    })
    .forget();
}
