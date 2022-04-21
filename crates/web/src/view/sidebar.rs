


use yew::prelude::*;
use yew_router::hooks::use_navigator;
use yewdux::prelude::*;

use crate::net::Client;

use crate::route::Route;
use crate::space::{Spaces};

#[function_component]
pub fn Sidebar() -> Html {
    let (spaces, _dispatch) = use_store::<Spaces>();
    let navigator = use_navigator().expect("Navigator not found");

    let list_spaces = spaces
        .iter()
        .map(|(address, _space)| {
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

                let mut title = address.to_string();
                title.truncate(3);

                html! {
                    <button class="p-4 shadow rounded-lg bg-slate-600" {onclick}>{title}</button>
                }
            };

            html! {
                { link }
            }
        })
        .collect::<Html>();

    html! {
        <div class="flex flex-col space-y-2 p-2 rounded-r ">
            { list_spaces }
            <CreateSpace />
        </div>
    }
}

#[function_component]
fn CreateSpace() -> Html {
    let client = use_store_value::<Client>();
    let navigator = use_navigator().expect("Navigator not found");
    let onclick = {
        let navigator = navigator;
        Dispatch::<Spaces>::new().reduce_callback(move |spaces| {
            let address = spaces.create_new_space();
            client.join_space(&address).ok();
            navigator.push(Route::Space {
                address,
            })
        })
    };
    html! {
        <button class="p-2 rounded-lg shadow bg-slate-600" {onclick}>{"+"}</button>
    }
}
