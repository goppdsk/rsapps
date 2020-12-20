use crate::domains::entities::user::User;
use crate::domains::errors::{ApplicationError, ErrorCode};
use crate::domains::ApplicationResult;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgreSQLUserRepository {
    pub db: PgPool,
}

impl PostgreSQLUserRepository {
    pub async fn get_all_users(&self) -> ApplicationResult<Vec<User>> {
        match sqlx::query_as(
            "
SELECT *
FROM users
            ",
        )
        .fetch_all(&self.db)
        .await
        {
            Ok(users) => Ok(users),
            Err(err) => Err(ApplicationError {
                code: ErrorCode::SystemError,
                message: "failed to fetch users".to_string(),
            }),
        }
    }
}
