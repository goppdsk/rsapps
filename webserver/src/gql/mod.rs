pub mod query;
pub mod user_resolver;

use crate::domains::errors::ApplicationError;
use crate::gql::query::QueryRoot;
use crate::State;
use juniper::http::{graphiql, GraphQLRequest, GraphQLResponse};
use juniper::{
    Context, EmptyMutation, EmptySubscription, FieldError, IntoFieldError, RootNode, ScalarValue,
};
use lazy_static::lazy_static;
use std::convert::AsRef;
use tide::http::mime;
use tide::{Body, Request, Response, StatusCode};

impl<S: ScalarValue> IntoFieldError<S> for ApplicationError {
    fn into_field_error(self) -> FieldError<S> {
        let code = self.code.as_ref();
        FieldError::new(
            self.message,
            graphql_value!({
              "code": code,
            }),
        )
    }
}

impl Context for State {}

type Schema = RootNode<'static, QueryRoot, EmptyMutation<State>, EmptySubscription<State>>;
lazy_static! {
    static ref SCHEMA: Schema =
        Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new());
}

pub async fn handle_graphql(mut request: Request<State>) -> tide::Result {
    let query: GraphQLRequest = request.body_json().await?;
    let response: GraphQLResponse = query.execute(&SCHEMA, request.state()).await;
    let status = if response.is_ok() {
        StatusCode::Ok
    } else {
        StatusCode::InternalServerError
    };

    Ok(Response::builder(status)
        .body(Body::from_json(&response)?)
        .build())
}

pub async fn handle_graphiql(_: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok(Response::builder(200)
        .body(graphiql::graphiql_source("/graphql", None))
        .content_type(mime::HTML))
}
