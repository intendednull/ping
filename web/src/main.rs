use yew::prelude::*;
use yew_services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};

use common::transport::{self as t, Message};

enum Msg {
    Response(Message),
    Request(Message),
    Noop,
    InitListener,
}

struct App {
    ws: Option<WebSocketTask>,
    link: ComponentLink<Self>,
}

impl App {
    fn request(&mut self, msg: &Message) {
        if let Some(ws) = &mut self.ws {
            ws.send_binary(Ok(t::pack(msg)));
        }
    }

    fn init_listener(&mut self) {
        let url = "ws://127.0.0.1:9001";
        log::debug!("Websocket connecting to {:?}", url);
        // Turn response into app message if possible.
        let handle_response = self
            .link
            .callback(|result: Result<Vec<u8>, anyhow::Error>| match result {
                Ok(data) => Msg::Response(t::unpack(&data)),
                Err(_) => Msg::Noop,
            });
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

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut this = Self {
            ws: Default::default(),
            link,
        };
        this.init_listener();

        this
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Response(resp) => {
                log::info!("{:?}", resp);
                false
            }
            Msg::Request(req) => {
                self.request(&req);
                false
            }
            Msg::InitListener => {
                self.init_listener();
                false
            }
            Msg::Noop => false,
        }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let onclick = self.link.callback(|_| Msg::Request(Message::Ping));
        html! {
            <button onclick=onclick>{"Ping"}</button>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::start_app::<App>();
}
