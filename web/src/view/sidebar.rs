use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;

use crate::net::Client;

use crate::route::Route;
use crate::space::{use_space, Action, Message, SpaceAddress, Spaces};

#[function_component]
pub fn Sidebar() -> Html {
    let (spaces, dispatch) = use_store::<Spaces>();
    let navigator = use_navigator().expect("Navigator not found");

    let list_spaces = spaces
        .iter()
        .map(|(address, space)| {
            let link = {
                let onclick = {
                    let address = address.clone();
                    let navigator = navigator.clone();
                    Callback::from(move |_| {
                        navigator.push(Route::Space {
                            address: address.clone(),
                        })
                    })
                };

                html! {
                    <button class="p-2 shadow " {onclick}>{address}</button>
                }
            };

            html! {
                { link }
            }
        })
        .collect::<Html>();

    html! {
        <div class="space-x-2">
            { list_spaces }
            <CreateSpace />
        </div>
    }
}

#[function_component]
fn CreateSpace() -> Html {
    let client = use_store_value::<Client>();
    let onclick = {
        Dispatch::<Spaces>::new().reduce_callback(move |spaces| {
            let address = spaces.create_new_space();
            client.join_space(&address).ok();
        })
    };
    html! {
        <button {onclick}>{"New space"}</button>
    }
}
