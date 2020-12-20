#[derive(AsRefStr, Debug)]
pub enum ErrorCode {
    UnAuthenticated,
    NoAuthHeaderError,
    InvalidAuthHeaderError,
    JWTTokenCreationError,
    Forbidden,
    NotFound,
    Duplicated,
    SystemError,
}

pub struct ApplicationError {
    pub code: ErrorCode,
    pub message: String,
}
