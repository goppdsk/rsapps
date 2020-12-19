use crate::domains::repositories::user_repository::UserRepository;
use crate::infrastructures::repositories::user_repository::PostgreSQLUserRepository;

use dyn_clone::DynClone;

pub trait DIContainer: DynClone + Send + Sync {
    fn user_repository(&self) -> Box<dyn UserRepository>;
}

dyn_clone::clone_trait_object!(DIContainer);

#[derive(Clone)]
pub struct PostgreSQLDIContainer;

impl DIContainer for PostgreSQLDIContainer {
    fn user_repository(&self) -> Box<dyn UserRepository> {
        Box::new(PostgreSQLUserRepository {})
    }
}
