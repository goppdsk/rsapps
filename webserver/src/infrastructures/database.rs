use sqlx::pool::Pool;
use sqlx::postgres::{PgPoolOptions, Postgres};

pub async fn create_pool(connections: u32, connection: &str) -> sqlx::Result<Pool<Postgres>> {
    PgPoolOptions::new()
        .max_connections(connections)
        .connect(connection)
        .await
}
