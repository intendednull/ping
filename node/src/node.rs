use std::collections::HashMap;

use lunatic::{
    process::{self, Process},
    Mailbox,
};
use serde::{Deserialize, Serialize};

use common::{
    address::Address,
    transport::{Request, Response},
};

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Request(Process<Response>, Request),
}

#[derive(Default)]
struct State {
    channels: HashMap<Address, Vec<Process<Response>>>,
}

pub fn start() -> Process<Msg> {
    process::spawn(|mailbox: Mailbox<Msg>| {
        let mut state = State::default();
        loop {
            if let Ok(msg) = mailbox.receive() {
                match msg {
                    Msg::Request(conn, msg) => match msg {
                        Request::Send(to, msg) => {
                            if let Some(channel) = state.channels.get(&to) {
                                for conn in channel {
                                    conn.send(Response(msg.clone()))
                                }
                            }
                        }
                        Request::Join(address) => {
                            let channel = state.channels.entry(address).or_default();
                            channel.push(conn);
                        }
                    },
                }
            }
        }
    })
    .expect("Unable to start node")
}
