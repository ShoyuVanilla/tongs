use yew::prelude::*;
use yew_hooks::use_mount;
use yew_router::prelude::*;

use crate::hooks::{use_user_context, UserState};
use crate::log;
use crate::routes::Route;

#[function_component(Home)]
pub fn home() -> Html {
    let user = match &*use_user_context() {
        UserState::LoggedIn(user) => user.clone(),
        UserState::Fetching => return html! { <></> },
        UserState::LoggedOut => {
            return html! {
                <Redirect<Route> to={Route::Login}/>
            }
        }
    };

    return html! {
        { "Home" }
    };
}
