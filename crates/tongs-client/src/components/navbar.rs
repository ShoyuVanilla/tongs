use crate::requests::auth::logout;
use yew::prelude::*;
use yew_hooks::use_async;
use yew_router::prelude::*;

use crate::hooks::{use_user_context, UserState};
use crate::log;
use crate::routes::Route;

#[function_component(Navbar)]
pub fn navbar() -> Html {
    let user_ctx = use_user_context();
    let user_ctx2 = user_ctx.clone();
    let user_ctx3 = user_ctx.clone();
    let ctx = use_state(move || user_ctx2);
    let logout = use_async(async move { logout().await });

    use_effect_with_deps(
        move |logout| {
            if logout.data.is_some() {
                user_ctx.logout();
            }
            || ()
        },
        logout.clone(),
    );
    use_effect_with_deps(move |_| || (), user_ctx3.clone());

    let onclick_logout = { move |_: MouseEvent| logout.run() };

    html! {
        <nav class="navbar is-fixed-top" role="navigation" aria-label="main navigation">
            <div class="navbar-brand">
                <div class="navbar-item">
                    <Link<Route> to={Route::Home}>{ "Tongs" }</Link<Route>>
                </div>
            </div>

            <div class="navbar-end">
                <div class="navbar-item">
                    { get_button(&*ctx, onclick_logout) }
                </div>
            </div>
        </nav>
    }
}

fn get_button<F: 'static + Fn(MouseEvent)>(user_state: &UserState, logout: F) -> Html {
    match user_state {
        UserState::LoggedIn(_user) => {
            html! {
                <button
                    class="button is-primary"
                    onclick={ logout }
                    >
                    { "Log out" }
                </button>
            }
        }
        _ => {
            html! {
                <div class="button">
                    <Link<Route> to={Route::Login}>{ "Login" }</Link<Route>>
                </div>
            }
        }
    }
}
