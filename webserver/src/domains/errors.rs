use serde::Serialize;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(AsRefStr, Debug, Serialize)]
pub enum ErrorCode {
    UnAuthenticated,
    NoAuthHeaderError,
    JWTTokenCreationError,
    NotFound,
    Conflict,
    OperationNameIsNotDefined,
    SystemError,
}

#[derive(Serialize)]
pub struct ApplicationError {
    pub code: ErrorCode,
    pub message: String,
}

impl Display for ApplicationError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "code: {:?}, message: {}", self.code, self.message)
    }
}
