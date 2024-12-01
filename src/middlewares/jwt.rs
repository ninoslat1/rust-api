use dotenv::dotenv;
use std::env;
use jsonwebtoken::{encode, decode, DecodingKey, EncodingKey, Header, Validation};
use crate::models::claims::Claims;

pub fn verify_token(token: &str) -> Result<bool, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::default();

    decode::<Claims>(&token, &decoding_key, &validation)?;

    Ok(true)
}