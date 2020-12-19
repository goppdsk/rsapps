use crate::domains::entities::context::RequestContext;
use crate::domains::entities::user::User;
use crate::infrastructures::di_container::DIContainer;

#[derive(Clone)]
pub struct UserService {
    pub di_container: Box<dyn DIContainer>,
}

impl UserService {
    fn get_all_users(self, context: RequestContext) -> std::io::Result<Vec<User>> {
        self.di_container.user_repository().get_all_users(context)
    }
}
