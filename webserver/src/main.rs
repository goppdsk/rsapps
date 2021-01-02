#[macro_use]
extern crate juniper;
#[macro_use]
extern crate strum;

mod auth;
mod domains;
mod gql;
mod infrastructures;
mod services;

use crate::gql::{handle_graphiql, handle_graphql};
use crate::infrastructures::database::create_pool;
use crate::infrastructures::di_container::PgDIContainer;
use crate::services::todo_service::TodoService;
use crate::services::user_service::UserService;
use std::env;
use std::sync::Arc;
use tide::http::headers::HeaderValue;
use tide::security::{CorsMiddleware, Origin};
use tide::{Redirect, Server};

#[derive(Clone)]
pub struct State {
    user_service: UserService,
    todo_service: TodoService,
}

async fn bootstrap(db_connections: &str) -> anyhow::Result<Server<State>> {
    let di_container = Arc::new(PgDIContainer {
        db: create_pool::<sqlx::Postgres>(5, db_connections).await?,
    });
    let cors = CorsMiddleware::new()
        .allow_methods("GET, POST, OPTIONS".parse::<HeaderValue>().unwrap())
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    let mut app = Server::with_state(State {
        user_service: UserService::new(di_container.as_ref()),
        todo_service: TodoService::new(di_container.as_ref()),
    });
    app.with(cors);
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    Ok(app)
}

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    tide::log::with_level(tide::log::LevelFilter::Info);
    let app = bootstrap(&env::var("DATABASE_URL")?).await?;
    app.listen("0.0.0.0:8081").await?;
    Ok(())
}
