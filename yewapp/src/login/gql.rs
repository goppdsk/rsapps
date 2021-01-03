use crate::utils::{request, FetchError};
use graphql_client::GraphQLQuery;
use wasm_bindgen::prelude::*;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "gql/schema.json", query_path = "gql/sign_up.graphql")]
pub struct SignUpUser;

#[derive(GraphQLQuery)]
#[graphql(schema_path = "gql/schema.json", query_path = "gql/login.graphql")]
pub struct Login;

pub async fn sign_up(username: String, password: String) -> Result<String, FetchError> {
    let request_body = SignUpUser::build_query(sign_up_user::Variables {
        user: sign_up_user::NewUser { username, password },
    });
    let resp = request::<sign_up_user::Variables>(request_body).await?;

    match resp.into_serde::<graphql_client::Response<sign_up_user::ResponseData>>() {
        Ok(data) => match data.data {
            Some(data) => Ok(data.sign_up),
            None => {
                return Err(FetchError {
                    err: JsValue::from_str("failed to sign up user"),
                })
            }
        },
        Err(_) => Err(FetchError {
            err: JsValue::from_str("failed to sign up user"),
        }),
    }
}

pub async fn login_with_username(username: String, password: String) -> Result<String, FetchError> {
    let request_body = Login::build_query(login::Variables { username, password });
    let resp = request::<login::Variables>(request_body).await?;

    match resp.into_serde::<graphql_client::Response<login::ResponseData>>() {
        Ok(data) => match data.data {
            Some(data) => Ok(data.login),
            None => {
                return Err(FetchError {
                    err: JsValue::from_str("failed to login up user"),
                })
            }
        },
        Err(_) => Err(FetchError {
            err: JsValue::from_str("failed to login up user"),
        }),
    }
}
