use actix_web::{web::Form, HttpResponse, Responder};
use sqlx::{ query, Row};
use dotenv::dotenv;
use std::env;
use base64::{encode_config, STANDARD};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde_json::json;
use cookie::{Cookie, SameSite, time};

use crate::libs::connection::connect_user;
use crate::models::login::{LoginForm, LoginResponse, ErrorResponse};
use crate::models::user::user;
use crate::models::stats::StatisticResponse;


pub async fn login(
    form: Form<LoginForm>,
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
    let admin_row = query(
        "SELECT UserName FROM myuser WHERE UserName = ? AND Password LIKE ?"
    )
    .bind(username)
    .bind(encoded_password)
    .fetch_optional(&pool)
    .await;

    if let Ok(Some(row)) = admin_row {
        let admin = user {
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
        
        let cookie = Cookie::build("SESSION_ID", access_token)
                    .http_only(true).secure(true).same_site(SameSite::Strict).max_age(time::Duration::days(7)).finish();

        return HttpResponse::Ok()
        .cookie(cookie)
        .json(LoginResponse {
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

pub async fn get_statistic() -> impl Responder {
    let pool = connect_user().await.unwrap();

    let stats = sqlx::query(r#"
        SELECT
            (SELECT COUNT(*) FROM lot) AS lot_count,
            (SELECT COUNT(*) FROM mycust) AS mycust_count,
            (SELECT COUNT(*) FROM mylocation) AS mylocation_count,
            (SELECT COUNT(*) FROM mypic) AS mypic_count,
            (SELECT COUNT(*) FROM area) AS area_count;
        "#)
        .fetch_one(&pool)
        .await
        .map_err(|e| {
            eprintln!("Gagal mengambil statistik dashboard: {}", e);
            HttpResponse::InternalServerError().finish()
        })
        .unwrap();

    HttpResponse::Ok().json(StatisticResponse {
        lot_count: stats.get(0),
        mycust_count: stats.get(1),
        mylocation_count: stats.get(2),
        mypic_count: stats.get(3),
        area_count: stats.get(4),
    })
}