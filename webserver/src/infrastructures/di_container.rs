use crate::domains::repositories::user_repository::UserRepository;
use crate::infrastructures::repositories::user_repository::PostgreSQLUserRepository;
use sqlx::PgPool;

pub trait DIContainer {
    fn user_repository(&self) -> Box<dyn UserRepository + Send + Sync>;
}

#[derive(Clone)]
pub struct PgDIContainer {
    pub db: PgPool,
}

impl DIContainer for PgDIContainer {
    fn user_repository(&self) -> Box<dyn UserRepository + Send + Sync> {
        Box::new(PostgreSQLUserRepository {
            db: self.db.clone(),
        })
    }
}
