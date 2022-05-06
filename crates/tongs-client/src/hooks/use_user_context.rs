use std::ops::Deref;
use yew::prelude::*;
use yew_router::prelude::*;

use crate::log;
use shared::UserInfo;

use crate::routes::Route;

#[derive(Clone, PartialEq)]
pub enum UserState {
    Fetching,
    LoggedIn(UserInfo),
    LoggedOut,
}

impl Default for UserState {
    fn default() -> Self {
        Self::Fetching
    }
}

pub struct UseUserContextHandle {
    inner: UseStateHandle<UserState>,
    history: AnyHistory,
}

impl UseUserContextHandle {
    pub fn login(&self, user_info: UserInfo) {
        self.inner.set(UserState::LoggedIn(user_info));
        self.history.push(Route::Admin);
    }

    pub fn logout(&self) {
        self.inner.set(UserState::LoggedOut);
        self.history.push(Route::Login)
    }
}

impl Deref for UseUserContextHandle {
    type Target = UserState;

    fn deref(&self) -> &Self::Target {
        &(*self.inner)
    }
}

impl Clone for UseUserContextHandle {
    fn clone(&self) -> Self {
        Self {
            inner: self.inner.clone(),
            history: self.history.clone(),
        }
    }
}

impl PartialEq for UseUserContextHandle {
    fn eq(&self, other: &Self) -> bool {
        *self.inner == *other.inner
    }
}

pub fn use_user_context() -> UseUserContextHandle {
    let inner = use_context::<UseStateHandle<UserState>>().unwrap();
    let history = use_history().unwrap();
    UseUserContextHandle { inner, history }
}
