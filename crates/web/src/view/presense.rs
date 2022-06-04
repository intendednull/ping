use std::rc::Rc;

use yew::prelude::*;

use yewdux::prelude::*;



use crate::space::{use_space, Space, SpaceAddress};

#[derive(Properties, Clone, PartialEq, Eq)]
pub struct Props {
    pub address: Rc<SpaceAddress>,
}

#[function_component]
pub fn ViewPresence(props: &Props) -> Html {
    let space = use_space(&props.address);
    let view = view_presense_list(&space);

    html! {
        <div class="flex flex-col space-y-2 p-2 rounded-r ">
            { view }
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
