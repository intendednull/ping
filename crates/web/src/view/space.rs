use std::rc::Rc;

use chrono::Utc;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yewdux::prelude::*;

use crate::net::Client;
use crate::space::{use_space, Action, Message, SpaceAddress};
use crate::view::presense::ViewPresence;

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub address: Rc<SpaceAddress>,
}

#[function_component]
pub fn ViewSpace(props: &Props) -> Html {
    html! {
        <div class="h-full flex">
            <div class="h-full flex-1">
                <div class="h-full flex flex-col">
                    <div class="overflow-scroll flex-1"><ViewMessages ..props.clone() /></div>
                    <div><InputMessage ..props.clone() /></div>
                </div>
            </div>
            <div class="overflow-scroll">
                <ViewPresence address={props.address.clone()} />
            </div>
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
                    .action(
                        &address,
                        Action::Message(Message {
                            text: input.value(),
                            author: client.peer.clone(),
                            timestamp: Utc::now(),
                        }),
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
            let alias = space
                .presense
                .get(&m.author)
                .map(|x| x.alias.clone())
                .unwrap_or_else(|| "anon".to_string());
            let timestamp = m.timestamp.format("%R").to_string();

            html! {
                <div class="p-2">
                    <div class="flex">
                        <p class="font-extralight">
                            {&alias}
                            {" "}
                            {&timestamp}
                        </p>
                    </div>
                    <p class="">{&m.text}</p>
                </div>
            }
        })
        .collect::<Html>()
}
