#[derive(Debug)]
pub enum AuthAPIError {
    UserAlreadyExists,
    InvalidCredentials,
    IncorrectCredentials,
    InvalidToken,
    MissingToken,
    UnexpectedError,
}
