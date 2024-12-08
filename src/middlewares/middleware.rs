
use actix_web::{
    body::BoxBody, dev::{ Service, ServiceRequest, ServiceResponse}, error::InternalError, Error, HttpResponse};
use jsonwebtoken::{decode, DecodingKey, Validation};
use dotenv::dotenv;
use std::env;

use crate::models::{claims::{Claims, SessionError, SessionErrorType}, login::ErrorResponse};

fn verify_token(token: &str) -> Result<bool, SessionError> {
    dotenv().ok();

    let secret_key = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    let decoding_key = DecodingKey::from_secret(secret_key.as_bytes());
    let validation = Validation::default();

    match decode::<Claims>(&token, &decoding_key, &validation) {
        Ok(_) => Ok(true),
        Err(err) => Err(SessionError {
            kind: SessionErrorType::TokenVerificationFailed,
            message: err.to_string(),
        }),
    }
}

pub async fn cookie_checkers<S>(
    req: ServiceRequest,
    service: &S,
) -> Result<ServiceResponse<BoxBody>, Error>
where
    S: Service<ServiceRequest, Response = ServiceResponse<BoxBody>, Error = InternalError<String>>,
{
    if req.path() != "/login" {
        let session_id = req
            .cookie("SESSION_ID")
            .map(|c| c.value().to_string())
            .unwrap_or_default();

        if !session_id.is_empty() {
            if verify_token(&session_id).is_ok() {
                let res = service.call(req).await?;
                return Ok(res);
            } else {
                let error_response = ErrorResponse {
                    message: "Invalid session token".to_string(),
                };
                let body = serde_json::to_vec(&error_response).unwrap();
                let res = HttpResponse::Unauthorized()
                    .body(body)
                    .map_into_boxed_body();
                return Ok(ServiceResponse::new(req.into_parts().0, res));
            }
        } else {
            let error_response = ErrorResponse {
                message: "No session token provided".to_string(),
            };
            let body = serde_json::to_vec(&error_response).unwrap();
            let res = HttpResponse::Unauthorized()
                .body(body)
                .map_into_boxed_body();
            return Ok(ServiceResponse::new(req.into_parts().0, res));
        }
    }

    let res = service.call(req).await?;
    Ok(res)
}