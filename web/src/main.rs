mod store;

use yew::prelude::*;
use yew_services::websocket::{WebSocketService, WebSocketStatus, WebSocketTask};
use yewdux::prelude::*;
use yewtil::NeqAssign;

use common::{
    channel::{Action, ChannelMsg},
    transport::{self as t, Request, Response},
};

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
            Request::Channel(ChannelMsg {
                id: "foo".to_owned(),
                hash: Default::default(),
                action: Action::Ping,
            })
        });
        let messages = self
            .dispatch
            .state()
            .channels
            .get(&"foo".to_owned())
            .map(|room| {
                room.state
                    .messages
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
