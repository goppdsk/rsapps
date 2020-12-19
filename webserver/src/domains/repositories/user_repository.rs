use crate::domains::entities::context::RequestContext;
use crate::domains::entities::user::User;

use dyn_clone::DynClone;

pub trait UserRepository: DynClone + Send + Sync {
    fn get_all_users(&self, context: RequestContext) -> std::io::Result<Vec<User>>;
}

dyn_clone::clone_trait_object!(UserRepository);
