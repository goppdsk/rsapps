#[derive(AsRefStr, Debug)]
pub enum ErrorCode {
    UnAuthenticated,
    Forbidden,
    NotFound,
    Duplicated,
    SystemError,
}

pub struct ApplicationError {
    pub code: ErrorCode,
    pub message: String,
}
