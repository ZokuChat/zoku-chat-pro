use std::sync::LazyLock;

use axum::{
    RequestPartsExt,
    extract::FromRequestParts,
    http::{StatusCode, request},
    response::IntoResponse,
};
use axum_extra::{
    TypedHeader,
    headers::{Authorization, authorization::Bearer},
};
use jsonwebtoken::{DecodingKey, EncodingKey, Validation, decode};
use serde::{Deserialize, Serialize};

pub static SECRET: LazyLock<Keys> = LazyLock::new(|| {
    let secret = std::env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    Keys::new(secret.as_bytes())
});

#[derive(Debug, Clone, Deserialize)]
pub struct Claims {
    pub userid: String,
}
impl<S> FromRequestParts<S> for Claims
where
    S: Send + Sync,
{
    type Rejection = AuthError;
    async fn from_request_parts(
        parts: &mut request::Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection> {
        let TypedHeader(Authorization(bearer)) = parts
            .extract::<TypedHeader<Authorization<Bearer>>>()
            .await
            .map_err(|_| AuthError::MissingCredentials)?;
        let token = decode::<Claims>(bearer.token(), &SECRET.decoding, &Validation::default())
            .map_err(|_| AuthError::TokenVerificationFailed)?;
        Ok(token.claims)
    }
}

#[derive(Debug, Serialize)]
pub struct AuthBody {
    pub access_token: String,
    pub token_type: String,
}
impl AuthBody {
    pub fn new(access_token: String) -> Self {
        Self {
            access_token,
            token_type: "Bearer".to_string(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct AuthPayload {
    pub password_hash: String,
    pub userid: String,
}

#[derive(Debug)]
pub enum AuthError {
    InvalidCredentials,
    MissingCredentials,
    TokenCreationFailed,
    TokenVerificationFailed,
    InternalError,
}
impl IntoResponse for AuthError {
    fn into_response(self) -> axum::response::Response {
        let (status, error_message) = match self {
            AuthError::InvalidCredentials => (StatusCode::UNAUTHORIZED, "Invalid credentials"),
            AuthError::MissingCredentials => (StatusCode::UNAUTHORIZED, "Missing credentials"),
            AuthError::TokenCreationFailed => {
                (StatusCode::INTERNAL_SERVER_ERROR, "Token creation failed")
            }
            AuthError::TokenVerificationFailed => {
                (StatusCode::UNAUTHORIZED, "Token verification failed")
            }
            AuthError::InternalError => (StatusCode::INTERNAL_SERVER_ERROR, "Internal error"),
        };
        let json = serde_json::json!({
            "error": error_message,
        });
        match serde_json::to_string(&json) {
            Ok(json) => (status, json).into_response(),
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Failed to serialize error message",
            )
                .into_response(),
        }
    }
}

pub struct Keys {
    pub encoding: EncodingKey,
    pub decoding: DecodingKey,
}

impl Keys {
    pub fn new(secret: &[u8]) -> Self {
        Self {
            encoding: EncodingKey::from_secret(secret),
            decoding: DecodingKey::from_secret(secret),
        }
    }
}
