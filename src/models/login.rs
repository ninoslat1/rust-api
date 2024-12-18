#[derive(serde::Deserialize)]
pub struct LoginForm {
    pub username: String,
    pub password: String,
}

#[derive(serde::Serialize)]
pub struct LoginResponse {
    pub message: String,
}

#[derive(serde::Serialize)]
pub struct ErrorResponse {
    pub message: String,
}