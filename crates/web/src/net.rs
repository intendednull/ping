use std::rc::Rc;

use async_std::sync::RwLock;
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::net::websocket::{futures::WebSocket, Message};
use wasm_bindgen_futures::spawn_local;
use yewdux::{dispatch, prelude::*};

use common::transport::{self, Input};

use crate::space::{self, SpaceAddress, Spaces};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Could not serialize message")]
    Serde,
}

pub fn init_msg_handler() {
    let client = dispatch::get::<Client>();
    spawn_local(async move {
        let dispatch = Dispatch::<Spaces>::new();
        while let Some(data) = client.read.write().await.next().await {
            match data {
                Ok(Message::Bytes(data)) => {
                    let output =
                        transport::unpack::<transport::Output>(&data).expect("Invalid output type");
                    let (action, peer_id) =
                        protocol::unpack::<space::Action>(&output.0).expect("Invalid msg type");

                    dispatch.reduce_mut(move |spaces| spaces.handle_action(action, peer_id))
                }
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
    pub identity: protocol::Identity,
    pub peer: protocol::Peer,
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
        let identity = protocol::Identity::new();

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
        self.send(&Input::Join(address.0.clone()))
    }

    pub fn action(
        &self,
        address: &SpaceAddress,
        action: &space::Action,
    ) -> Result<(), ClientError> {
        let data = protocol::pack(action, self.identity.clone()).map_err(|_| ClientError::Serde)?;
        self.send(&Input::Send(address.0.clone(), data.into()))
    }

    fn send(&self, input: &Input) -> Result<(), ClientError> {
        let write = self.write.clone();
        let data = transport::pack(&input).map_err(|_| ClientError::Serde)?;

        spawn_local(async move {
            let result = write.write().await.send(Message::Bytes(data)).await;

            if let Err(err) = result {
                log::error!("Error sending request: {}", err)
            }
        });

        Ok(())
    }
}
