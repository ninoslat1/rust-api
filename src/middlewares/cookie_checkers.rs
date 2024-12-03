use actix_web::{HttpRequest, HttpResponse, Result, Error, dev::ServiceRequest, dev::ServiceResponse};
use actix_web::actix_service::Service;
use jsonwebtoken::{decode, DecodingKey, Validation};
use std::env;
use dotenv::dotenv;
use crate::models::claims::Claims;

fn verify_token(token: &str) -> Result<bool, jsonwebtoken::errors::Error> {
    dotenv().ok();

    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::default();

    decode::<Claims>(&token, &decoding_key, &validation)?;

    Ok(true)
}

pub async fn cookie_middleware(req: ServiceRequest, next: &dyn actix_service::Service) -> Result<ServiceResponse, Error> {
    dotenv().ok(); // Load .env file

    // Extract the SESSIONID cookie
    if let Some(cookie) = req.cookies().find(|c| c.name() == "SESSIONID") {
        let token = cookie.value();

        // Verify the token
        match verify_token(token) {
            Ok(_) => {
                // Token is valid, proceed with the request
                return Ok(next.call(req).await?);
            },
            Err(_) => {
                // Token is invalid, return Unauthorized
                return Ok(req.error_response(HttpResponse::Unauthorized()));
            }
        }
    }

    // No token found, return Unauthorized
    Ok(req.error_response(HttpResponse::Unauthorized()))
}