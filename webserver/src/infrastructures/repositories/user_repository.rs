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

    async fn get_user_by_username(&self, username: String) -> anyhow::Result<Option<User>> {
        Ok(sqlx::query_as!(
            User,
            "
SELECT *
FROM users
WHERE username = $1
            ",
            username
        )
        .fetch_optional(&self.db)
        .await?)
    }

    async fn create_user(&self, user: User) -> anyhow::Result<User> {
        Ok(sqlx::query_as!(
            User,
            "
INSERT INTO users (username, email, password_hash, created_at, updated_at)
VALUES ($1, $2, $3, $4, $5)
returning *
            ",
            user.username,
            user.email,
            user.password_hash,
            user.created_at,
            user.updated_at,
        )
        .fetch_one(&self.db)
        .await?)
    }
}
