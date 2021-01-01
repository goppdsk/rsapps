use sqlx::{pool::PoolOptions, Database, Pool};
use std::env;

pub async fn get_db<DB: Database>() -> anyhow::Result<Pool<DB>> {
    Ok(PoolOptions::<DB>::new()
        .max_connections(1)
        .connect(&env::var("DATABASE_URL")?)
        .await?)
}
