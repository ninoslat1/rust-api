use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub exp: DateTime<Utc>,
}

#[derive(Debug)]
pub struct SessionError {
    pub kind: SessionErrorType,
    pub message: String,
}

#[derive(Debug)]
pub enum SessionErrorType {
    InvalidSession,
    TokenVerificationFailed,
}