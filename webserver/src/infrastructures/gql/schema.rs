use crate::domains::errors::{ApplicationError, ErrorCode};
use crate::domains::models::user::User;
use juniper::http::{graphiql, GraphQLRequest, GraphQLResponse};
use juniper::{
    Context, EmptyMutation, EmptySubscription, FieldError, IntoFieldError, RootNode, ScalarValue,
};
use lazy_static::lazy_static;
use std::convert::AsRef;
use std::sync::{Arc, RwLock};
use tide::http::mime;
use tide::{Body, Redirect, Request, Response, Server, StatusCode};

impl<S: ScalarValue> IntoFieldError<S> for ApplicationError {
    fn into_field_error(self) -> FieldError<S> {
        let code = self.code.as_ref();
        match self.code {
            ErrorCode::SystemError => FieldError::new(
                self.message,
                graphql_value!({
                  "code": code,
                }),
            ),
            _ => FieldError::new(
                "Un exptected error is occured",
                graphql_value!({
                  "code": "UnExptected",
                }),
            ),
        }
    }
}

#[graphql_object]
#[graphql(description = "A user")]
impl User {
    #[graphql(description = "A user id")]
    fn id(&self) -> i32 {
        self.id.unwrap_or(0)
    }

    #[graphql(description = "A user name")]
    fn username(&self) -> &str {
        &self.username
    }

    #[graphql(description = "A user password")]
    fn password(&self) -> &str {
        &self.password
    }

    #[graphql(description = "A user email")]
    fn email(&self) -> &str {
        &self.email
    }
}

#[derive(Clone)]
pub struct State {}

impl Context for State {}

pub struct QueryRoot;

#[graphql_object(Context = State)]
impl QueryRoot {
    #[graphql(name = "apiVersion")]
    fn api_version() -> &str {
        "0.1.0"
    }

    #[graphql(description = "Get all Users")]
    fn users(context: &State) -> Vec<User> {
        let users: Vec<User> = Vec::new();
        users.iter().cloned().collect()
    }
}

pub type Schema = RootNode<'static, QueryRoot, EmptyMutation<State>, EmptySubscription<State>>;
lazy_static! {
    static ref SCHEMA: Schema =
        Schema::new(QueryRoot {}, EmptyMutation::new(), EmptySubscription::new());
}

async fn handle_graphql(mut request: Request<State>) -> tide::Result {
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

async fn handle_graphiql(_: Request<State>) -> tide::Result<impl Into<Response>> {
    Ok(Response::builder(200)
        .body(graphiql::graphiql_source("/graphql", None))
        .content_type(mime::HTML))
}

pub fn create_graphql_server() -> Server<State> {
    let mut app = Server::with_state(State {});
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    app
}
