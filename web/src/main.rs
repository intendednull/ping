pub mod net;
pub mod space;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use net::Client;
use space::{Action, Message, Spaces};

#[function_component]
fn App() -> Html {
    let client = use_store_value::<Client>();
    let (spaces, spaces_dispatch) = use_store::<Spaces>();
    let mut it = spaces.iter();
    if let Some((address, space)) = it.next() {
        let messages = space
            .messages()
            .map(|m| html! { <p>{&m.text}</p> })
            .collect::<Html>();

        let input = {
            let onkeypress = {
                let client = client;
                let address = address.clone();
                Callback::from(move |e: KeyboardEvent| {
                    if e.key() == "Enter" {
                        let input = e.target_unchecked_into::<HtmlInputElement>();
                        client
                            .send(
                                &address,
                                &net::Msg::Space(Action::Send(
                                    address.clone(),
                                    Message {
                                        text: input.value(),
                                    },
                                )),
                            )
                            .unwrap();

                        input.set_value("");
                    }
                })
            };
            html! {
                <input {onkeypress} />
            }
        };

        html! {
            <>
            { input }
            { messages }
            </>

        }
    } else {
        let onclick = {
            let client = client;
            spaces_dispatch.reduce_callback(move |spaces| {
                let address = spaces.create_new_space();
                client.join_space(&address).ok();
            })
        };
        html! {
            <button {onclick}>{"New space"}</button>
        }
    }
}

fn main() {
    net::init_msg_handler();
    space::join_spaces();
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
