use crate::domains::entities::user::User;
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository {
    async fn get_all_users(&self) -> sqlx::Result<Vec<User>>;

    async fn get_user_by_id(&self, id: i32) -> sqlx::Result<Option<User>>;

    async fn get_user_by_email(&self, email: String) -> sqlx::Result<Option<User>>;
}
