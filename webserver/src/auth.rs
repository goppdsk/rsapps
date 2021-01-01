use crate::domains::errors::{ApplicationError, ErrorCode};
use crate::domains::ApplicationResult;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use tide::http::headers::AUTHORIZATION;
use tide::http::Headers;

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    sub: String,
    exp: usize,
}

const BEARER: &str = "Bearer";

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

pub fn create_jwt(uid: i32) -> ApplicationResult<String> {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(1))
        .expect("valid timestamp")
        .timestamp();
    let claims = Claims {
        sub: uid.to_string(),
        exp: expiration as usize,
    };
    let headers = Header::new(Algorithm::HS512);
    encode(
        &headers,
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_bytes()),
    )
    .map_err(|_| ApplicationError {
        code: ErrorCode::JWTTokenCreationError,
        message: "faied to create JWT token".to_owned(),
    })
}

pub fn get_jwt_claims(headers: Headers) -> ApplicationResult<Claims> {
    let header = &headers[AUTHORIZATION];
    let auth_header = match header.get(0) {
        Some(v) => v.as_str(),
        None => {
            return Err(ApplicationError {
                code: ErrorCode::NoAuthHeaderError,
                message: "auth header is invalid".to_owned(),
            })
        }
    };
    if !auth_header.starts_with(BEARER) {
        return Err(ApplicationError {
            code: ErrorCode::NoAuthHeaderError,
            message: "auth header is invalid".to_owned(),
        });
    }
    match decode::<Claims>(
        auth_header.trim_start_matches(BEARER),
        &DecodingKey::from_secret(JWT_SECRET.as_bytes()),
        &Validation::default(),
    ) {
        Ok(claims) => Ok(claims.claims),
        Err(_) => Err(ApplicationError {
            code: ErrorCode::UnAuthenticated,
            message: "token is invalid".to_owned(),
        }),
    }
}
