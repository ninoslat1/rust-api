use actix_web::{web, HttpResponse, Responder};
use sqlx::Row;
use dotenv::dotenv;
use std::env;
use base64::{encode_config, STANDARD};
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use argon2::{password_hash::{rand_core::OsRng, SaltString, PasswordHash, PasswordHasher, PasswordVerifier},Algorithm::Argon2id};
use serde_json::json;

use crate::libs::connection::connect_user;
use crate::models::login::{LoginForm, LoginResponse, ErrorResponse};
use crate::models::user::user_name;

pub async fn login(
    form: web::Form<LoginForm>,
) -> impl Responder {
    let username = &form.username;
    let password = &form.password;
    let mut buffer = vec![];

    for c in password.as_bytes().iter().cloned() {
        buffer.push(c);
        buffer.push(0);
    }

    let encoded_password = encode_config(&buffer, STANDARD);
    let pool = connect_user().await.unwrap();

    // Jalankan query
    let admin_row = sqlx::query(
        "SELECT UserName FROM myuser WHERE UserName = ? AND Password LIKE ?",
    )
    .bind(username)
    .bind(encoded_password)
    .fetch_optional(&pool)
    .await;



    if let Ok(Some(row)) = admin_row {
        // Map hasil Row ke struct User
        let admin = user_name {
            ID: None,
            UserName: row.get("UserName"),
            UserCode: None,
            Password: None,
            Position: None,
            Telephone: None,
            Email: None,
            Handphone: None,
            GroupID: None,
            LogIn: None,
            SecurityCode: None,
            Status: None,
            UserID: None,
        };

        let access_token = generate_token(&admin.UserName);
        return HttpResponse::Ok().json(LoginResponse {
            access_token,
            message: "Login berhasil".to_string(),
        });
    }

    return HttpResponse::NotFound().json(ErrorResponse {
        message: "Pengguna tidak ada".to_string(),
    })
}

fn generate_token(username: &str) -> String {
    dotenv().ok();

    let secret_key = env::var("APP_TOKEN").expect("APP_TOKEN must be set");
    let encoding_key = EncodingKey::from_secret(secret_key.as_bytes());
    let claims = json!({ "sub": username });

    encode(&Header::default(), &claims, &encoding_key).unwrap()
}