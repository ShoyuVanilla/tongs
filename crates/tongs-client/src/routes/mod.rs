mod home;
mod login;

use yew::prelude::*;
use yew_router::prelude::*;

use home::Home;
use login::Login;

#[derive(Clone, PartialEq, Routable)]
pub enum Route {
    #[at("/")]
    Home,
    #[at("/login")]
    Login,
    #[at("/admin")]
    Admin,
    #[not_found]
    #[at("/404")]
    NotFound,
}

pub fn switch(route: &Route) -> Html {
    match route {
        Route::Home => html! { <Home /> },
        Route::Login => html! { <Login /> },
        Route::Admin => html! { "Admin" },
        Route::NotFound => html! { "404 not found!" },
    }
}
