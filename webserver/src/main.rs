#[macro_use]
extern crate juniper;
#[macro_use]
extern crate strum;

mod domains;
mod gql;
mod infrastructures;
mod services;

use crate::domains::errors::{ApplicationError, ErrorCode};
use crate::domains::ApplicationResult;
use crate::gql::{handle_graphiql, handle_graphql};
use crate::infrastructures::database::create_pool;
use crate::infrastructures::di_container::DIContainer;
use crate::services::user_service::UserService;
use chrono::Utc;
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::env;
use tide::http::headers::AUTHORIZATION;
use tide::http::Headers;
use tide::{Redirect, Server};

#[derive(Clone)]
pub struct State {
    user_service: UserService,
}

/// Our claims struct, it needs to derive `Serialize` and/or `Deserialize`
#[derive(Debug, Serialize, Deserialize)]
struct Claims {
    sub: String,
    exp: usize,
}

const BEARER: &str = "Bearer";

lazy_static! {
    static ref JWT_SECRET: String = env::var("JWT_SECRET").unwrap();
}

fn create_jwt(uid: i32) -> ApplicationResult<String> {
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

fn get_jwt_claims(headers: Headers) -> ApplicationResult<Claims> {
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

async fn bootstrap(db_connections: &str) -> tide::Result<Server<State>> {
    let di_container = Box::new(DIContainer {
        db: create_pool(5, db_connections).await?,
    });
    let mut app = Server::with_state(State {
        user_service: UserService::new(di_container),
    });
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    Ok(app)
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);
    let app = bootstrap("postgres://postgres:P@ssw0rd!@localhost:15432/rsapps").await?;
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
