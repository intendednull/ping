use yew::prelude::*;
use yewdux::prelude::*;

use common::{
    channel::{Action, ChannelMsg},
    transport::{self as t, Request, Response},
};

#[function_component]
fn App() -> Html {
    html! {}
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
