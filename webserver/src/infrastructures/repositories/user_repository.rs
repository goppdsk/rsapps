use crate::domains::entities::context::RequestContext;
use crate::domains::entities::user::User;
use crate::domains::repositories::user_repository::UserRepository;

#[derive(Clone)]
pub struct PostgreSQLUserRepository;

impl UserRepository for PostgreSQLUserRepository {
    fn get_all_users(&self, context: RequestContext) -> std::io::Result<Vec<User>> {
        Ok(Vec::new())
    }
}
