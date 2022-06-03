use std::rc::Rc;

use async_std::sync::RwLock;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yewdux::{dispatch, prelude::*};

use crate::space::{self, SpaceAddress, Spaces};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Could not serialize message")]
    Serde,
    #[error("Protocol error")]
    Protocal(#[from] protocol::Error),
}

pub fn init_msg_handler() {
    let client = dispatch::get::<Client>();
    spawn_local(async move {
        let dispatch = Dispatch::<Spaces>::new();
        while let Some(data) = client.read.write().await.next().await {
            match data {
                Ok(Message::Bytes(data)) => match protocol::unpack::<space::Action>(&data) {
                    Ok((action, peer_id, address)) => dispatch.reduce_mut(move |spaces| {
                        spaces.handle_action(action, peer_id, &SpaceAddress(address))
                    }),
                    Err(e) => {
                        log::error!("Error receiving message: {:?}", e);
                    }
                },
                Err(_e) => {}
                _ => {}
            }
        }
    })
}

#[derive(Clone)]
pub struct Client {
    pub write: Rc<RwLock<SplitSink<WebSocket, Message>>>,
    pub read: Rc<RwLock<SplitStream<WebSocket>>>,
    pub identity: protocol::identity::Identity,
    pub peer: protocol::identity::Peer,
}

impl PartialEq for Client {
    fn eq(&self, _other: &Self) -> bool {
        true
    }
}

impl Store for Client {
    fn new() -> Self {
        let ws = WebSocket::open("ws://localhost:9001").unwrap();
        let (write, read) = ws.split();
        let identity = protocol::identity::Identity::new();

        Self {
            write: RwLock::new(write).into(),
            read: RwLock::new(read).into(),
            peer: identity.as_peer(),
            identity,
        }
    }
}

impl Client {
    pub fn join_space(&self, address: &SpaceAddress) -> Result<(), ClientError> {
        self.send(protocol::Message::JoinRoom(address.0.clone()))
    }

    pub fn action(&self, address: &SpaceAddress, action: space::Action) -> Result<(), ClientError> {
        self.send(protocol::Message::Peer(
            protocol::Payload::Public(action),
            self.identity.clone(),
            address.0.clone(),
        ))
    }

    fn send(&self, msg: protocol::Message<space::Action>) -> Result<(), ClientError> {
        let write = self.write.clone();
        let payload = protocol::pack(msg)?;

        spawn_local(async move {
            let result = write.write().await.send(Message::Bytes(payload)).await;

            if let Err(err) = result {
                log::error!("Error sending request: {}", err)
            }
        });

        Ok(())
    }
}
