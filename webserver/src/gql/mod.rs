pub(crate) mod mutation;
pub(crate) mod query;
pub(crate) mod todo_resolver;
pub(crate) mod user_resolver;

use crate::auth;
use crate::domains::errors::ApplicationError;
use crate::gql::mutation::MutationRoot;
use crate::gql::query::QueryRoot;
use crate::State;
use juniper::http::{graphiql, GraphQLRequest, GraphQLResponse};
use juniper::{Context, EmptySubscription, FieldError, IntoFieldError, RootNode, ScalarValue};
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

pub struct GraphQLContext {
    state: State,
    user_id: i32,
}

impl Context for GraphQLContext {}

type Schema = RootNode<'static, QueryRoot, MutationRoot, EmptySubscription<GraphQLContext>>;
lazy_static! {
    static ref SCHEMA: Schema =
        Schema::new(QueryRoot {}, MutationRoot {}, EmptySubscription::new());
}

pub async fn handle_graphql(mut request: Request<State>) -> tide::Result<impl Into<Response>> {
    let query: GraphQLRequest = request.body_json().await?;
    let claim = match auth::get_jwt_claims(request.header(tide::http::headers::AUTHORIZATION)) {
        Ok(c) => c,
        Err(_) => {
            return Ok(Response::builder(StatusCode::BadRequest).build());
        }
    };
    let gql_ctx = GraphQLContext {
        state: request.state().clone(),
        user_id: claim.sub.parse::<i32>().unwrap(),
    };
    let response: GraphQLResponse = query.execute(&SCHEMA, &gql_ctx).await;
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
