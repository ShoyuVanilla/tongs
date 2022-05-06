use crate::hooks::use_user_context;
use crate::log;
use shared::{LoginInfo, LoginInfoWrapper, UserInfo};
use wasm_bindgen_futures::spawn_local;
use web_sys::HtmlInputElement;
use yew::prelude::*;
use yew_hooks::use_async;

#[function_component(Login)]
pub fn login() -> Html {
    let user_ctx = use_user_context();
    let login_info = use_state(LoginInfo::default);
    let user_login = {
        let login_info = login_info.clone();
        use_async(async move {
            let payload = LoginInfoWrapper {
                user: (*login_info).clone(),
            };
            crate::requests::auth::login(payload).await
        })
    };

    use_effect_with_deps(
        move |user_login| {
            if let Some(user_info) = &user_login.data {
                user_ctx.login(user_info.user.clone());
            }
            || ()
        },
        user_login.clone(),
    );

    let onsubmit = {
        Callback::from(move |e: FocusEvent| {
            e.prevent_default();
            user_login.run();
        })
    };

    let oninput_username = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.username = input.value();
            login_info.set(info);
        })
    };

    let oninput_password = {
        let login_info = login_info.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut info = (*login_info).clone();
            info.password = input.value();
            login_info.set(info);
        })
    };

    html! {
        <div class="container">
            <div class="columns is-centered">
                <form class="box column is-one-third" { onsubmit }>
                    <div class="field">
                        <label class="label">{ "Username" }</label>
                        <div class="control">
                            <input
                                class="input"
                                type="text"
                                autocomplete="on"
                                placeholder="admin"
                                value={ login_info.username.clone() }
                                oninput={ oninput_username }
                                />
                        </div>
                    </div>

                    <div class="field">
                        <label class="label">{ "Password" }</label>
                        <div class="control">
                            <input
                                class="input"
                                type="password"
                                autocomplete="on"
                                placeholder="********"
                                value={ login_info.password.clone() }
                                oninput={ oninput_password }
                                />
                        </div>
                    </div>

                    <button
                        class="button is-primary"
                        type="submit"
                        >
                        { "Sign in" }
                    </button>
                </form>
            </div>
        </div>
    }
}
