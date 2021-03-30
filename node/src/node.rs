use std::collections::HashMap;

use serde::{Deserialize, Serialize};
use slab::Slab;
use wactor::*;

use common::transport::{Group, Request, Response};

use crate::client::Responder;

#[derive(Deserialize, Serialize)]
pub enum Input {
    Request { client_id: usize, msg: Request },
    RegisterClient(Bridge<Responder>),
}

#[derive(Deserialize, Serialize)]
pub enum Output {
    /// Receive a message from client.
    Response(Response),
    /// Assign an internal id to the client. This is **not** a unique identifier for individual
    /// users, and will be reused when connection closes.
    ClientId(usize),
}

pub struct Node {
    /// Tracking connected clients.
    clients: Slab<Bridge<Responder>>,
    groups: HashMap<String, Vec<Bridge<Responder>>>,
}

impl Node {
    fn join_group(&mut self, client_id: usize, group_id: String) {
        if let Some(client) = self.clients.get(client_id) {
            self.groups
                .entry(group_id)
                .or_default()
                .push(client.clone())
        }
    }

    fn send(&self, client_id: usize, msg: Response) {
        if let Some(client) = self.clients.get(client_id) {
            client.send(msg).ok();
        }
    }

    fn send_group(&mut self, group: Group) {
        if let Some(clients) = self.groups.get_mut(&group.id) {
            clients.retain(|client| client.send(Response::Group(group.clone())).is_ok())
        }
    }
}

impl Actor for Node {
    type Input = Input;
    type Output = Output;

    fn create() -> Self {
        Self {
            clients: Default::default(),
            groups: Default::default(),
        }
    }

    fn handle(&mut self, msg: Self::Input, link: &Link<Self>) {
        match msg {
            Input::RegisterClient(responder) => {
                let id = self.clients.insert(responder);
                link.respond(Output::ClientId(id)).ok();
            }
            Input::Request { client_id, msg } => match msg {
                Request::JoinRoom(id) => {
                    self.join_group(client_id, id);
                }
                Request::Group(group) => self.send_group(group),
            },
        }
    }
}

impl From<Response> for Output {
    fn from(val: Response) -> Self {
        Output::Response(val)
    }
}
