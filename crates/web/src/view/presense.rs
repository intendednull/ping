use std::rc::Rc;

use web_sys::HtmlInputElement;
use yew::prelude::*;

use yewdux::prelude::*;

use crate::{
    presense::Presense,
    space::{use_space, Space, SpaceAddress},
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
                    <h1 class="text-xl"><ViewAlias /></h1>
                </div>
            </div>
        </div>
    }
}

fn view_presense_list(space: &Space) -> Html {
    space
        .presense
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
pub fn ViewAlias() -> Html {
    let alias = use_selector(|x: &Presense| x.alias.clone());
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

    let onchange = Dispatch::<Presense>::new().reduce_mut_callback_with(move |s, e: Event| {
        s.alias = e.target_unchecked_into::<HtmlInputElement>().value();
        is_editing.set(false);
    });

    html! {
        <input class="bg-slate-800 rounded p-3" value={alias.to_string()} {onchange} />
    }
}
