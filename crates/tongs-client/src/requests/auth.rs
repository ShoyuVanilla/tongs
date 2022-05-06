use crate::log;
use gloo_net::http::Request;
use shared::{LoginInfoWrapper, UserInfoWrapper};

#[derive(Clone, PartialEq)]
pub struct AuthError {}

pub async fn current_user() -> Result<UserInfoWrapper, AuthError> {
    let resp = Request::get("/api/user/current").send().await;
    if resp.is_err() {
        return Err(AuthError {});
    }
    let resp = resp.unwrap();
    let json: Result<UserInfoWrapper, gloo_net::Error> = resp.json().await;
    match json {
        Ok(json) => Ok(json),
        Err(e) => Err(AuthError {}),
    }
}

pub async fn login(payload: LoginInfoWrapper) -> Result<UserInfoWrapper, AuthError> {
    let resp = Request::post("/api/login")
        .json(&payload)
        .unwrap()
        .send()
        .await;
    if resp.is_err() {
        return Err(AuthError {});
    }
    let resp = resp.unwrap();
    let json: Result<UserInfoWrapper, gloo_net::Error> = resp.json().await;
    match json {
        Ok(json) => Ok(json),
        Err(e) => Err(AuthError {}),
    }
}

pub async fn logout() -> Result<(), AuthError> {
    if let Ok(resp) = Request::post("/api/user/logout").send().await {
        match resp.status() {
            200 => Ok(()),
            _ => Err(AuthError {}),
        }
    } else {
        Err(AuthError {})
    }
}
