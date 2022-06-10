use std::rc::Rc;

use chrono::{Duration, Utc};
use gloo::timers::callback::Timeout;
use web_sys::HtmlInputElement;
use yew::prelude::*;

use yewdux::{dispatch, prelude::*};

use crate::net::Client;
use crate::presense::PRESENSE_INTERVAL;
use crate::space::{use_space, Action, Message, SpaceAddress, Universe};
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
            } else {
                // Set is_typing
                {
                    let address = address.clone();
                    Dispatch::<Universe>::new().reduce_mut(move |s| {
                        let space = s.space_mut(&address);
                        space.presense.local.is_typing = true;
                        space.presense.local.last_updated = Utc::now();
                    });
                }
                // Unset is_typing if no activity for 5 seconds.
                {
                    let address = address.clone();
                    Timeout::new(PRESENSE_INTERVAL * 1000, move || {
                        Dispatch::<Universe>::new().reduce_mut(move |s| {
                            let space = s.space_mut(&address);
                            if Utc::now() - space.presense.local.last_updated
                                >= Duration::seconds(PRESENSE_INTERVAL as _)
                            {
                                space.presense.local.is_typing = false;
                                space.presense.local.last_updated = Utc::now();
                            }
                        });
                    })
                    .forget();
                }
            }
        })
    };

    let typing_indicator = {
        let names = use_selector_with_deps(
            |s: &Universe, address| {
                let local_id = dispatch::get::<Client>().peer.clone();
                s.get(address)
                    .map(|space| {
                        space
                            .presense
                            .peers
                            .values()
                            .filter(|x| x.peer_id != local_id)
                            .filter(|x| x.is_typing)
                            .map(|x| x.alias.clone())
                            .collect::<Vec<_>>()
                            .join(", ")
                    })
                    .unwrap_or_default()
            },
            props.address.clone(),
        );

        if names.is_empty() {
            html! {}
        } else {
            html! { <>{ names }{" is typing" }</> }
        }
    };
    html! {
        <div class="p-4 sticky">
            { typing_indicator }
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
                .peers
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
