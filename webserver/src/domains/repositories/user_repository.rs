use crate::domains::entities::user::User;
use async_trait::async_trait;
use dyn_clone::DynClone;

#[async_trait]
pub trait UserRepository: DynClone {
    async fn get_all_users(&self) -> anyhow::Result<Vec<User>>;

    async fn get_user_by_id(&self, id: i32) -> anyhow::Result<Option<User>>;

    async fn get_user_by_email(&self, email: String) -> anyhow::Result<Option<User>>;

    async fn get_user_by_username(&self, username: String) -> anyhow::Result<Option<User>>;

    async fn create_user(&self, user: User) -> anyhow::Result<User>;
}

dyn_clone::clone_trait_object!(UserRepository);
