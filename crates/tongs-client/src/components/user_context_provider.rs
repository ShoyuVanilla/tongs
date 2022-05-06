use yew::prelude::*;
use yew_hooks::{use_async, use_mount};

use crate::hooks::UserState;
use crate::log;
use crate::requests::auth::current_user;
use shared::UserInfo;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(UserContextProvider)]
pub fn user_context_provider(props: &Props) -> Html {
    let user_ctx: UseStateHandle<UserState> = use_state(|| UserState::default());
    let current_user = use_async(async move { current_user().await });
    use_state(|| user_ctx.clone());

    {
        let current_user = current_user.clone();
        use_mount(move || {
            current_user.run();
        });
    }

    {
        let user_ctx = user_ctx.clone();
        use_effect_with_deps(
            move |current_user| {
                if let Some(user_info) = &current_user.data {
                    user_ctx.set(UserState::LoggedIn(user_info.user.clone()))
                }
                // TODO: Error handling => Erase user
                || {}
            },
            current_user,
        );
    }

    return html! {
        <ContextProvider<UseStateHandle<UserState>> context={ user_ctx }>
            { for props.children.iter() }
        </ContextProvider<UseStateHandle<UserState>>>
    };
}
