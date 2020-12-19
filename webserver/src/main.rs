#[macro_use]
extern crate juniper;
#[macro_use]
extern crate strum;
#[macro_use]
extern crate dyn_clone;

mod domains;
mod gql;
mod infrastructures;
mod services;

use crate::gql::schema::{handle_graphiql, handle_graphql};
use crate::infrastructures::di_container::PostgreSQLDIContainer;
use crate::services::user_service::UserService;
use tide::{Redirect, Server};

#[derive(Clone)]
pub struct State {
    user_service: UserService,
}

fn bootstrap() -> Server<State> {
    let mut app = Server::with_state(State {
        user_service: UserService {
            di_container: Box::new(PostgreSQLDIContainer {}),
        },
    });
    app.at("/").get(Redirect::permanent("/graphiql"));
    app.at("/graphql").post(handle_graphql);
    app.at("/graphiql").get(handle_graphiql);
    app
}

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let app = bootstrap();
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
