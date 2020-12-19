#[macro_use]
extern crate juniper;
#[macro_use]
extern crate strum;

mod domains;
mod infrastructures;

use infrastructures::gql::schema::create_graphql_server;

#[async_std::main]
async fn main() -> std::io::Result<()> {
    let app = create_graphql_server();
    app.listen("0.0.0.0:8080").await?;
    Ok(())
}
