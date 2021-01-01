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
use crate::services::user_service::UserService;
use std::sync::Arc;
use tide::{Redirect, Server};

#[derive(Clone)]
pub struct State {
    user_service: UserService,
}

async fn bootstrap(db_connections: &str) -> tide::Result<Server<State>> {
    let di_container = Arc::new(PgDIContainer {
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
