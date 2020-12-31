use crate::domains::entities::user::User;
use crate::domains::repositories::user_repository::UserRepository;
use async_trait::async_trait;
use sqlx::PgPool;

#[derive(Clone)]
pub struct PostgreSQLUserRepository {
    pub db: PgPool,
}

#[async_trait]
impl UserRepository for PostgreSQLUserRepository {
    async fn get_all_users(&self) -> sqlx::Result<Vec<User>> {
        sqlx::query_as(
            "
SELECT *
FROM users
            ",
        )
        .fetch_all(&self.db)
        .await
    }

    async fn get_user_by_id(&self, id: i32) -> sqlx::Result<Option<User>> {
        sqlx::query_as(
            "
SELECT *
FROM users
WHERE id = $1
            ",
        )
        .bind(id)
        .fetch_optional(&self.db)
        .await
    }

    async fn get_user_by_email(&self, email: String) -> sqlx::Result<Option<User>> {
        sqlx::query_as(
            "
SELECT *
FROM users
WHERE email = $1
            ",
        )
        .bind(email)
        .fetch_optional(&self.db)
        .await
    }
}
