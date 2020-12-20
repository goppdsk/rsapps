use crate::domains::entities::user::User;
use sqlx::query::QueryAs;
use sqlx::Database;

use crate::domains::ApplicationResult;

pub trait UserRepository<DB: Database>: Send + Sync {
    fn get_all_users(&self) -> ApplicationResult<QueryAs<DB, User, User>>;
}
