pub mod entities;
pub mod errors;
pub mod repositories;

use errors::ApplicationError;

pub type ApplicationResult<T> = Result<T, ApplicationError>;
