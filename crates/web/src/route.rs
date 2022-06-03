use std::rc::Rc;

use yew::prelude::*;
use yew_router::prelude::*;
use yewdux::prelude::*;

use crate::{space::SpaceAddress, view};

#[derive(Routable, PartialEq, Clone, Debug)]
pub enum Route {
    #[at("/space/:address")]
    Space { address: SpaceAddress },
    #[at("/")]
    Home,
}

impl Store for Route {
    fn new() -> Self {
        Route::Home
    }
}

pub fn switch(route: Route) -> Html {
    Dispatch::<Route>::new().set(route.clone());

    match route {
        Route::Home => html! {},
        Route::Space { address } => html! {
            <view::ViewSpace address={Rc::new(address.clone())} />
        },
    }
}
