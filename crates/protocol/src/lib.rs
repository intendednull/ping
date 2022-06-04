use common::address::Address;
use identity::{Identity, PeerId};
use serde::{de::DeserializeOwned, Deserialize, Serialize};

pub mod identity;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Serde error")]
    Serde,
    #[error("Identity error")]
    Identity(#[from] identity::Error),
}

#[derive(Serialize, Deserialize)]
pub enum Payload<T> {
    Public(T),
    Private(T),
}

pub enum Message<T> {
    Peer(Payload<T>, Identity, Address),
    JoinRoom(Address),
}

pub fn pack<T: Serialize>(msg: Message<T>) -> Result<Vec<u8>, Error> {
    match msg {
        Message::Peer(payload, ident, address) => match payload {
            Payload::Public(payload) => {
                let payload = Payload::Public(identity::pack(&payload, ident)?);
                let payload = common::transport::pack(&payload).map_err(|_| Error::Serde)?;
                common::transport::pack(&common::transport::Input::Send(address, payload.into()))
                    .map_err(|_e| Error::Serde)
            }
            Payload::Private(_payload) => {
                unimplemented!()
            }
        },
        Message::JoinRoom(address) => {
            common::transport::pack(&common::transport::Input::Join(address))
                .map_err(|_e| Error::Serde)
        }
    }
}

pub fn unpack<T: DeserializeOwned>(payload: &[u8]) -> Result<(T, PeerId, Address), Error> {
    let output: common::transport::Output =
        common::transport::unpack(payload).map_err(|_| Error::Serde)?;
    let payload: Payload<Vec<u8>> =
        common::transport::unpack(&output.payload).map_err(|_| Error::Serde)?;

    match payload {
        Payload::Public(payload) => {
            let (result, peer_id) = identity::unpack(&payload)?;

            Ok((result, peer_id, output.address))
        }
        Payload::Private(_payload) => {
            unimplemented!()
        }
    }
}
