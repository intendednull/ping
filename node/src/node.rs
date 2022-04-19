use std::collections::HashMap;

use lunatic::{
    process::{self, Process},
    Mailbox,
};
use serde::{Deserialize, Serialize};

use common::{
    address::Address,
    transport::{Input, Output},
};

#[derive(Serialize, Deserialize)]
pub enum Msg {
    Request(Process<Output>, Input),
}

#[derive(Default)]
struct State {
    channels: HashMap<Address, Vec<Process<Output>>>,
}

pub fn start() -> Process<Msg> {
    process::spawn(|mailbox: Mailbox<Msg>| {
        let mut state = State::default();
        loop {
            if let Ok(msg) = mailbox.receive() {
                match msg {
                    Msg::Request(conn, msg) => match msg {
                        Input::Send(to, msg) => {
                            if let Some(channel) = state.channels.get(&to) {
                                for conn in channel {
                                    conn.send(Output(msg.clone()))
                                }
                            }
                        }
                        Input::Join(address) => {
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
