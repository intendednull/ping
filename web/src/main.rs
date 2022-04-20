pub mod net;
pub mod route;
pub mod space;
pub mod view;

use yew::prelude::*;
use yew_router::prelude::*;

use route::{switch, Route};
use view::Sidebar;

#[function_component]
fn App() -> Html {
    html! {
        <>
        <BrowserRouter>
            <Sidebar />
            <main>
                <Switch<Route> render={Switch::render(switch)} />
            </main>
        </BrowserRouter>
        </>
    }
}

fn main() {
    net::init_msg_handler();
    space::join_spaces();
    wasm_logger::init(wasm_logger::Config::default());
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
