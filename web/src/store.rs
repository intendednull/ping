use std::collections::HashMap;
use std::rc::Rc;

use serde::{Deserialize, Serialize};
use yew::prelude::*;
use yew_services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yewdux::prelude::{self, *};

use common::{
    block::Block,
    channel::{Action, ChannelMsg},
    transport::{self as t, Request, Response},
};

#[derive(Clone, Default, Serialize)]
pub struct ChannelState {
    pub messages: Vec<String>,
    pub participants: usize,
}

impl ChannelState {
    fn apply(&mut self, action: Action) {
        match action {
            Action::Participants(n) => {
                self.participants = n;
            }
            Action::Ping => {
                self.messages.push("ping".into());
            }
            Action::Pong => {
                self.messages.push("pong".into());
            }
        }
    }
}

#[derive(Clone)]
pub struct Channel {
    pub state: ChannelState,
    pub block: Block<Action>,
}

impl Channel {
    fn new(action: Action) -> anyhow::Result<Self> {
        let mut state = ChannelState::default();
        state.apply(action.clone());

        Ok(Self {
            block: Block::new(action, &state)?,
            state,
        })
    }

    fn apply(&mut self, action: Action) -> anyhow::Result<()> {
        self.state.apply(action.clone());
        self.block = self.block.update(action, &self.state)?;

        Ok(())
    }
}

#[derive(Clone, Default)]
pub struct State {
    pub channels: HashMap<String, Channel>,
}

impl State {
    fn apply(&mut self, msg: ChannelMsg) -> anyhow::Result<()> {
        match self.channels.get_mut(&msg.id) {
            Some(channel) if channel.block.hash == msg.hash => {
                channel.apply(msg.action)?;
            }
            Some(_channel) => {
                log::debug!("Mismatched action");
            }
            None => {
                self.channels.insert(msg.id, Channel::new(msg.action)?);
            }
        }

        Ok(())
    }
}

pub enum Input {
    Request(Request),
}

pub enum Output {}

pub enum Msg {
    Response(Response),
    InitListener,
}

pub struct Store {
    state: Rc<State>,
    ws: Option<WebSocketTask>,
    link: StoreLink<Self>,
}

impl prelude::Store for Store {
    type Model = State;
    type Input = Input;
    type Output = Output;
    type Message = Msg;

    fn new(link: StoreLink<Self>) -> Self {
        link.send_message(Msg::InitListener);

        Self {
            state: Default::default(),
            ws: Default::default(),
            link,
        }
    }

    fn state(&mut self) -> &mut Rc<Self::Model> {
        &mut self.state
    }

    fn update(&mut self, msg: Self::Message) -> Changed {
        match msg {
            Msg::Response(msg) => match msg {
                Response::Channel(msg) => {
                    self.state_mut().apply(msg).ok();
                    true
                }
            },
            Msg::InitListener => {
                self.init_listener();
                false
            }
        }
    }

    fn handle_input(&mut self, msg: Self::Input, _who: HandlerId) -> Changed {
        match msg {
            Input::Request(req) => {
                self.request(&req);
                false
            }
        }
    }
}

impl Store {
    fn state_mut(&mut self) -> &mut State {
        Rc::make_mut(&mut self.state)
    }

    fn request(&mut self, msg: &Request) {
        if let Some(ws) = &mut self.ws {
            log::info!("Sending: {:?}", msg);
            ws.send_binary(t::pack(msg));
        }
    }

    fn init_listener(&mut self) {
        let url = "ws://127.0.0.1:9001";
        log::debug!("Websocket connecting to {:?}", url);
        // Turn response into app message if possible.
        let handle_response = {
            let on_response = self.link.callback(Msg::Response);
            Callback::from(move |result: Result<Vec<u8>, anyhow::Error>| {
                match result.as_ref().map(t::unpack) {
                    Ok(Ok(data)) => {
                        log::info!("Received: {:?}", data);
                        on_response.emit(data)
                    }
                    _ => {}
                }
            })
        };
        // If connection is closed, try to reconnect.
        let reconnect = self.link.callback(|_| Msg::InitListener);
        let handle_status = Callback::from(move |status| {
            log::debug!("{:?}", status);
            match status {
                WebSocketStatus::Closed | WebSocketStatus::Error => {
                    reconnect.emit(());
                }
                _ => {}
            }
        });
        // Save task to keep it alive. Drops previous task, so we always only have one.
        self.ws = Some(
            WebSocketService::connect_binary(&url, handle_response, handle_status)
                .expect("Websocket task"),
        );
    }
}

impl From<Request> for Input {
    fn from(val: Request) -> Self {
        Input::Request(val)
    }
}
