mod components;
mod hooks;
mod requests;
mod routes;

use wasm_bindgen::prelude::*;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::components::navbar::Navbar;
use crate::components::user_context_provider::UserContextProvider;
use crate::routes::{switch, Route};

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <UserContextProvider>
            <BrowserRouter>
                <Navbar />
                <Switch<Route> render={Switch::render(switch)} />
            </BrowserRouter>
        </UserContextProvider>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::start_app::<Application>();
}

#[cfg(debug_assertions)]
#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
}
