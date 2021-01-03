#[derive(AsRefStr, Debug)]
pub enum ErrorCode {
    UnAuthenticated,
    NoAuthHeaderError,
    JWTTokenCreationError,
    NotFound,
    Conflict,
    SystemError,
}

pub struct ApplicationError {
    pub code: ErrorCode,
    pub message: String,
}
