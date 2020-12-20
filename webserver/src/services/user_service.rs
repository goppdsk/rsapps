use crate::domains::entities::user::User;
use crate::domains::ApplicationResult;
use crate::infrastructures::di_container::DIContainer;

#[derive(Clone)]
pub struct UserService {
    pub di_container: Box<DIContainer>,
}

impl UserService {
    pub async fn get_all_users(self) -> ApplicationResult<Vec<User>> {
        self.di_container.user_repository().get_all_users().await
    }
}
