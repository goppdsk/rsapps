use crate::domains::entities::user::User;
use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait UserRepository: DynClone {
    async fn get_all_users(&self) -> sqlx::Result<Vec<User>>;

    async fn get_user_by_id(&self, id: i32) -> sqlx::Result<Option<User>>;

    async fn get_user_by_email(&self, email: String) -> sqlx::Result<Option<User>>;
}

dyn_clone::clone_trait_object!(UserRepository);
