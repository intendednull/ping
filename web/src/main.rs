mod store;

use yew::prelude::*;
use yew_services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yewdux::prelude::*;
use yewtil::NeqAssign;

use common::transport::{self as t, ChainMsg, Channel, Request, Response};

struct App {
    dispatch: DispatchProps<store::Store>,
}

impl Component for App {
    type Message = ();
    type Properties = DispatchProps<store::Store>;

    fn create(dispatch: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { dispatch }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        false
    }

    fn change(&mut self, dispatch: Self::Properties) -> ShouldRender {
        self.dispatch.neq_assign(dispatch)
    }

    fn view(&self) -> Html {
        let join = self
            .dispatch
            .callback(|_| Request::JoinChannel("foo".to_owned()));
        let onclick = self.dispatch.callback(|_| {
            Request::Channel(Channel {
                id: "foo".to_owned(),
                action: ChainMsg::Ping,
            })
        });
        let messages = self
            .dispatch
            .state()
            .rooms
            .get(&"foo".to_owned())
            .map(|room| {
                room.messages
                    .iter()
                    .map(|m| html! { <p>{ m }</p> })
                    .collect::<Html>()
            })
            .unwrap_or_default();

        html! {
            <>
            <button onclick=join>{"Join"}</button>
            <button onclick=onclick>{"Ping"}</button>
            { messages }
            </>
        }
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::start_app::<WithDispatch<App>>();
}
