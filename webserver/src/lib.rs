#[macro_use]
extern crate juniper;
#[macro_use]
extern crate strum;

pub mod auth;
pub mod domains;
pub mod gql;
pub mod infrastructures;
pub mod services;

use crate::services::todo_service::TodoService;
use crate::services::user_service::UserService;

#[derive(Clone)]
pub struct State {
    pub user_service: UserService,
    pub todo_service: TodoService,
}
