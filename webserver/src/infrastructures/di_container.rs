use crate::domains::repositories::todo_repository::TodoRepository;
use crate::domains::repositories::user_repository::UserRepository;
use crate::infrastructures::repositories::todo_repository::PostgreSQLTodoRepository;
use crate::infrastructures::repositories::user_repository::PostgreSQLUserRepository;

pub trait DIContainer {
    fn user_repository(&self) -> Box<dyn UserRepository + Send + Sync>;
    fn todo_repository(&self) -> Box<dyn TodoRepository + Send + Sync>;
}

#[derive(Clone)]
pub struct PgDIContainer {
    pub db: sqlx::PgPool,
}

impl DIContainer for PgDIContainer {
    fn user_repository(&self) -> Box<dyn UserRepository + Send + Sync> {
        Box::new(PostgreSQLUserRepository {
            db: self.db.clone(),
        })
    }

    fn todo_repository(&self) -> Box<dyn TodoRepository + Send + Sync> {
        Box::new(PostgreSQLTodoRepository {
            db: self.db.clone(),
        })
    }
}
