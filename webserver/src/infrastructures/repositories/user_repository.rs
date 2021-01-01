use crate::domains::entities::user::User;
use crate::domains::repositories::user_repository::UserRepository;
use async_trait::async_trait;

#[derive(Clone)]
pub struct PostgreSQLUserRepository {
    pub db: sqlx::PgPool,
}

#[async_trait]
impl UserRepository for PostgreSQLUserRepository {
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>> {
        Ok(sqlx::query_as!(
            User,
            "
SELECT *
FROM users
            ",
        )
        .fetch_all(&self.db)
        .await?)
    }

    async fn get_user_by_id(&self, id: i32) -> anyhow::Result<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            "
SELECT *
FROM users
WHERE id = $1
            ",
            id
        )
        .fetch_optional(&self.db)
        .await?)
    }

    async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            "
SELECT *
FROM users
WHERE email = $1
            ",
            email
        )
        .fetch_optional(&self.db)
        .await?)
    }
}
