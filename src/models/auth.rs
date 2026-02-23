use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Serialize)]
pub struct LoginResponse {
    pub token: String,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    pub sub: String,
    pub email: String,
    pub is_admin: bool,
    pub exp: usize,
}
