use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::net::Client;

use crate::space::{use_space, Action, Message, SpaceAddress};

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub address: Rc<SpaceAddress>,
}

#[function_component]
pub fn ViewSpace(props: &Props) -> Html {
    html! {
        <div class="h-full flex flex-col">
            <div class="overflow-scroll flex-1"><ViewMessages ..props.clone() /></div>
            <div><InputMessage ..props.clone() /></div>
        </div>
    }
}

#[function_component]
fn InputMessage(props: &Props) -> Html {
    let client = use_store_value::<Client>();
    let onkeypress = {
        let address = props.address.clone();
        Callback::from(move |e: KeyboardEvent| {
            if e.key() == "Enter" {
                let input = e.target_unchecked_into::<HtmlInputElement>();
                client
                    .send(
                        &address,
                        &crate::net::Msg::Space(Action::Send(
                            address.as_ref().clone(),
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
        <div class="p-4 sticky">
            <input class="p-3 bg-slate-800 shadow-lg rounded-lg w-full" {onkeypress} />
        </div>
    }
}

#[function_component]
fn ViewMessages(props: &Props) -> Html {
    let space = use_space(&props.address);
    space
        .messages()
        .map(|m| {
            html! {
                <p class="p-2">{&m.text}</p>
            }
        })
        .collect::<Html>()
}
