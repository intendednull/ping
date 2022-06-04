pub mod net;
pub mod presense;
pub mod route;
pub mod space;
pub mod view;

use yew::prelude::*;
use yew_router::prelude::*;

use route::{switch, Route};
use view::{Sidebar};

#[function_component]
fn App() -> Html {
    html! {
        <BrowserRouter>
            <div class="flex bg-slate-700 text-white h-full">
                <Sidebar />
                <main class="flex-1">
                    <Switch<Route> render={switch} />
                </main>
            </div>
        </BrowserRouter>
    }
}

fn main() {
    net::init_msg_handler();
    presense::init();

    space::join_spaces();
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
