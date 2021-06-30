use chrono::ParseError as ChronoParseError;
use diesel::result::Error as DieselResultError;
use jsonwebtoken::errors::Error as JWTError;
use redis::RedisError;
use serde_json::error::Error as SerdeError;
use thiserror::Error;
use tonic::Status;
use tracing::error;

#[derive(Error, Debug)]
pub enum UserServerError {
    #[error("there are some problem while : {0}")]
    DatabaseError(String),
    #[error("argument error : {0}")]
    ArgumentError(String),
    #[error("validation error : {0}")]
    ValidationError(String),
    #[error("json parse error : {0}")]
    JsonParseError(String),
    #[error("not found: {0}")]
    NotFound(String),
    #[error("password hash error : {0}")]
    PasswordHashError(String),
    #[error("password unauthorized : {0}")]
    PasswordUnauthorizedError(String),
    #[error("jwt generation error : {0}")]
    JWTGenerationError(String),
    #[error("jwt verify error : {0}")]
    JWTVerifyError(String),
    #[error("redis error : {0}")]
    RedisError(String),
}

impl From<SerdeError> for UserServerError {
    fn from(err: SerdeError) -> Self {
        UserServerError::JsonParseError(err.to_string())
    }
}

impl From<DieselResultError> for UserServerError {
    fn from(err: DieselResultError) -> Self {
        UserServerError::DatabaseError(err.to_string())
    }
}

impl From<JWTError> for UserServerError {
    fn from(_: JWTError) -> Self {
        UserServerError::JWTVerifyError("unauthorized token".to_string())
    }
}

impl From<ChronoParseError> for UserServerError {
    fn from(err: ChronoParseError) -> Self {
        UserServerError::ArgumentError(err.to_string())
    }
}

impl From<RedisError> for UserServerError {
    fn from(err: RedisError) -> Self {
        UserServerError::RedisError(err.to_string())
    }
}

impl From<UserServerError> for Status {
    fn from(error: UserServerError) -> Self {
        match error {
            UserServerError::DatabaseError(message) => Status::internal(message),
            UserServerError::NotFound(message) => Status::not_found(message),
            UserServerError::ArgumentError(message) => Status::invalid_argument(message),
            UserServerError::ValidationError(message) => Status::invalid_argument(message),
            UserServerError::JsonParseError(message) => Status::unavailable(message),
            UserServerError::JWTVerifyError(message) => Status::unauthenticated(message),
            UserServerError::PasswordUnauthorizedError(message) => Status::unauthenticated(message),
            _ => Status::internal("Internal Server Error".to_string()),
        }
    }
}
