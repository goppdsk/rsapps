pub(crate) mod entities;
pub(crate) mod errors;
pub(crate) mod repositories;

use errors::ApplicationError;

pub type ApplicationResult<T> = Result<T, ApplicationError>;
