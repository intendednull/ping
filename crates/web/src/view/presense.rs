use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use yewdux::prelude::*;

use crate::{
    space::{use_space, Space, SpaceAddress, Universe},
};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub address: Rc<SpaceAddress>,
}

#[function_component]
pub fn ViewPresence(props: &Props) -> Html {
    let space = use_space(&props.address);
    let view = view_presense_list(&space);

    html! {
        <div class="flex flex-col justify-between h-full">
            <div class="flex flex-col space-y-2 p-2">
                { view }
            </div>
            <div class="p-4">
                <div class="p-3 bg-slate-600 rounded shadow">
                    <h1 class="text-xl"><ViewAlias ..props.clone() /></h1>
                </div>
            </div>
        </div>
    }
}

fn view_presense_list(space: &Space) -> Html {
    space
        .presense
        .peers
        .values()
        .map(|presense| {
            let alias = &presense.alias;
            html! {
                <p>{ alias }</p>
            }
        })
        .collect::<Html>()
}

#[function_component]
pub fn ViewAlias(props: &Props) -> Html {
    let alias = use_selector_with_deps(
        |state: &Universe, address| {
            state
                .get(address)
                .map(|space| space.presense.local.alias.clone())
                .unwrap_or_else(|| "anon".to_string())
        },
        props.address.clone(),
    );

    let is_editing = use_state(|| false);
    let onclick = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| is_editing.set(true))
    };

    if !*is_editing {
        return html! {
            <p {onclick}>{&alias}</p>
        };
    }

    let onchange = {
        let address = props.address.clone();
        Dispatch::<Universe>::new().reduce_mut_callback_with(move |s, e: Event| {
            s.space_mut(&address).presense.local.alias =
                e.target_unchecked_into::<HtmlInputElement>().value();

            is_editing.set(false);
        })
    };

    html! {
        <input class="bg-slate-800 rounded p-3" value={alias.to_string()} {onchange} />
    }
}
