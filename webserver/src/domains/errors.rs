#[derive(AsRefStr, Debug)]
pub enum ErrorCode {
    UnAuthenticated,
    NoAuthHeaderError,
    JWTTokenCreationError,
    NotFound,
    SystemError,
}

pub struct ApplicationError {
    pub code: ErrorCode,
    pub message: String,
}
