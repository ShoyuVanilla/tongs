use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Application)]
pub fn app() -> Html {
    html! {
        <p>{ "Hello World!" }</p>
    }
}

#[wasm_bindgen(start)]
pub fn start() {
    yew::start_app::<Application>();
}
