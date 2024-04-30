use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]

pub struct LoginInfo {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub success: bool,
    pub token: String,
    // pub state: Vec<i32>,
}

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
}

#[derive(Deserialize)]
pub struct UserCreateInfo {
    pub email: String,
    pub username: String,
    pub password: String,
    pub age: u8,
}

pub struct UserCreateResponse {
    pub success: bool,
    pub user_id: u32,
}

pub struct User {
    id: u16,
    username: String,
    password: String,
    email: String,
}
