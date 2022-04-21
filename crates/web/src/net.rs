use std::rc::Rc;

use async_std::sync::RwLock;
use common::transport::{self, Input};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gloo::net::websocket::{futures::WebSocket, Message};
use serde::{Deserialize, Serialize};
use wasm_bindgen_futures::spawn_local;

use yewdux::{dispatch, prelude::*};

use crate::space::{self, SpaceAddress, Spaces};

#[derive(Debug, thiserror::Error)]
pub enum ClientError {
    #[error("Could not serialize message")]
    Serde,
}

#[derive(PartialEq, Serialize, Deserialize, Debug, Clone)]
pub enum Msg {
    Space(space::Action),
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
                    let msg = transport::unpack::<Msg>(&output.0).expect("Invalid msg type");

                    match msg {
                        Msg::Space(action) => {
                            dispatch.reduce(move |spaces| spaces.handle_action(action))
                        }
                    };
                }
                _ => {}
            }
        }
    })
}

#[derive(Clone)]
pub struct Client {
    pub write: Rc<RwLock<SplitSink<WebSocket, Message>>>,
    pub read: Rc<RwLock<SplitStream<WebSocket>>>,
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

        Self {
            write: RwLock::new(write).into(),
            read: RwLock::new(read).into(),
        }
    }
}

impl Client {
    pub fn join_space(&self, address: &SpaceAddress) -> Result<(), ClientError> {
        self.request(&Input::Join(address.0.clone()))
    }

    pub fn send(&self, address: &SpaceAddress, msg: &Msg) -> Result<(), ClientError> {
        let data = transport::pack(msg).map_err(|_| ClientError::Serde)?;
        self.request(&Input::Send(address.0.clone(), data.into()))
    }

    fn request(&self, input: &Input) -> Result<(), ClientError> {
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
