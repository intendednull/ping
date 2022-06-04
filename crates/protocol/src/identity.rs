use std::rc::Rc;

use libp2p::identity::{Keypair, PublicKey};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use common::transport;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Serde error")]
    Serde,
    #[error("Invalid signature")]
    InvalidSignature,
    #[error("Unable to sign payload")]
    SignError,
    #[error("Unable to decode public key")]
    PublicKey,
}

#[derive(Clone)]
pub struct Identity(Rc<Keypair>);
impl Identity {
    pub fn as_peer(&self) -> PeerId {
        PeerId(self.0.public().to_peer_id().into())
    }
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct PeerId(Rc<libp2p::PeerId>);

impl Identity {
    pub fn new() -> Self {
        Self(Keypair::generate_ed25519().into())
    }
}

#[derive(Serialize, Deserialize)]
struct OpaquePublicKey(Vec<u8>);
#[derive(Serialize, Deserialize)]
pub struct Signature(Vec<u8>);

#[derive(Serialize, Deserialize)]
struct Message {
    public_key: OpaquePublicKey,
    signature: Signature,
    payload: Vec<u8>,
}

impl Message {
    fn verify(&self) -> Result<libp2p::PeerId, Error> {
        let public_key =
            PublicKey::from_protobuf_encoding(&self.public_key.0).map_err(|_| Error::PublicKey)?;

        let verified = public_key.verify(&self.payload, &self.signature.0);
        if verified {
            Ok(public_key.to_peer_id())
        } else {
            Err(Error::InvalidSignature)
        }
    }
}

pub fn pack<T: Serialize>(payload: &T, identity: Identity) -> Result<Vec<u8>, Error> {
    let payload = transport::pack(payload).map_err(|_| Error::Serde)?;
    let signature = Signature(identity.0.sign(&payload).map_err(|_| Error::Serde)?);
    let public_key = OpaquePublicKey(identity.0.public().to_protobuf_encoding());

    transport::pack(&Message {
        public_key,
        signature,
        payload,
    })
    .map_err(|_| Error::Serde)
}

pub fn unpack<T: DeserializeOwned>(payload: &[u8]) -> Result<(T, PeerId), Error> {
    let message: Message = transport::unpack(payload).map_err(|_| Error::Serde)?;
    let peer_id = message.verify()?;
    let payload = transport::unpack(&message.payload).map_err(|_| Error::Serde)?;

    Ok((payload, PeerId(peer_id.into())))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn pack_and_unpack_works() {
        let identity = Identity::new();
        let payload = 1;

        let data = pack(&payload, identity.clone()).unwrap();
        let (result, peer) = unpack::<i32>(&data).unwrap();

        assert!(peer.0.is_public_key(&identity.0.public()).unwrap());
        assert_eq!(result, payload)
    }
}
