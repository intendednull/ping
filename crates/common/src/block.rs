use serde::Serialize;
use sha3::{digest, Digest, Sha3_256};

use crate::transport;

#[derive(Clone, Serialize)]
pub struct Block<M: Serialize> {
    pub msg: M,
    pub hash: digest::Output<Sha3_256>,
    pub previous_hash: Option<digest::Output<Sha3_256>>,
}

impl<M: Serialize> Block<M> {
    pub fn new<S: Serialize>(msg: M, state: &S) -> anyhow::Result<Self> {
        let hash = Sha3_256::new()
            .chain(transport::pack(&msg)?)
            .chain(transport::pack(state)?)
            .finalize();

        Ok(Self {
            msg,
            hash,
            previous_hash: None,
        })
    }

    pub fn update<S: Serialize>(&self, msg: M, state: &S) -> anyhow::Result<Self> {
        let hash = Sha3_256::new()
            .chain(transport::pack(&msg)?)
            .chain(transport::pack(state)?)
            .chain(self.hash)
            .finalize();

        Ok(Self {
            msg,
            hash,
            previous_hash: Some(self.hash),
        })
    }
}
