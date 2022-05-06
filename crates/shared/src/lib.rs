use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Deserialize, Serialize)]
pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Deserialize, Serialize)]
pub struct LoginInfoWrapper {
    pub user: LoginInfo,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct UserInfo {
    pub username: String,
}

#[derive(Clone, Deserialize, Serialize, PartialEq)]
pub struct UserInfoWrapper {
    pub user: UserInfo,
}
