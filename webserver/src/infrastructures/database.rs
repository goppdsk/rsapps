use sqlx::{pool::PoolOptions, Database, Pool};

pub async fn create_pool<DB: Database>(
    connections: u32,
    connection: &str,
) -> anyhow::Result<Pool<DB>> {
    Ok(PoolOptions::<DB>::new()
        .max_connections(connections)
        .connect(connection)
        .await?)
}
