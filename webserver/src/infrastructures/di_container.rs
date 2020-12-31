use crate::domains::repositories::user_repository::UserRepository;
use crate::infrastructures::repositories::user_repository::PostgreSQLUserRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct DIContainer {
    pub db: PgPool,
}

impl DIContainer {
    pub fn user_repository(&self) -> Box<dyn UserRepository + Send + Sync> {
        Box::new(PostgreSQLUserRepository {
            db: self.db.clone(),
        })
    }
}
