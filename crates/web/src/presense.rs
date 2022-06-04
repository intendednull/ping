use gloo::timers::callback::Interval;
use serde::{Deserialize, Serialize};

use chrono::{DateTime, Duration, Utc};
use protocol::identity::{PeerId};

use yewdux::{
    dispatch,
    prelude::{Dispatch, Store},
};

use crate::{
    net::Client,
    space::{Space, Universe},
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Presense {
    pub peer_id: PeerId,
    pub alias: String,
    last_updated: DateTime<Utc>,
}

impl Store for Presense {
    fn new() -> Self {
        Self {
            peer_id: dispatch::get::<Client>().peer.clone(),
            alias: "Anonymous".to_string(),
            last_updated: Utc::now(),
        }
    }
}

impl Presense {
    pub fn apply(&self, space: &mut Space) {
        let mut presense = self.clone();
        presense.last_updated = Utc::now();

        space.presense.insert(self.peer_id.clone(), presense);
    }
}

pub fn init() {
    Interval::new(5_000, move || {
        let client = dispatch::get::<Client>();
        let universe = dispatch::get::<Universe>();
        let presense = dispatch::get::<Presense>();
        for (address, space) in universe.iter() {
            // Send presense data to space.
            client
                .action(
                    address,
                    crate::space::Action::Presense(presense.as_ref().clone()),
                )
                .ok();
            // Get all presense data that has expired.
            let expired = space
                .presense
                .values()
                .filter(|x| Utc::now() - x.last_updated > Duration::seconds(10))
                .map(|x| x.peer_id.clone());
            // Remove stale presense data.
            Dispatch::<Universe>::new().reduce_mut(|universe| {
                for peer in expired {
                    universe.space_mut(address).presense.remove(&peer);
                }
            });
        }
    })
    .forget();
}
