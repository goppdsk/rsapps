use crate::infrastructures::repositories::user_repository::PostgreSQLUserRepository;
use sqlx::PgPool;

#[derive(Clone)]
pub struct DIContainer {
    pub db: PgPool,
}

impl DIContainer {
    pub fn user_repository(&self) -> Box<PostgreSQLUserRepository> {
        Box::new(PostgreSQLUserRepository {
            db: self.db.clone(),
        })
    }
}
