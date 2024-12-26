use api::error::{Error, ErrorKind};
use redis::RedisError;
use serde::Serializer;

pub type ServerResult<T> = Result<T, ServerError>;

#[derive(Debug, thiserror::Error)]
pub enum ServerError {
    #[error(transparent)]
    RedisError(#[from] RedisError),

    #[error("unsupported connection type")]
    UnsupportedConnType,

    #[error(transparent)]
    IoError(#[from] std::io::Error),

}

impl serde::Serialize for ServerError {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let err_msg = self.to_string();
        let err = match self {
            ServerError::RedisError(_) => Error {
                msg: err_msg,
                kind: ErrorKind::Redis,
            },
            ServerError::UnsupportedConnType => Error {
                msg: err_msg,
                kind: ErrorKind::UnsupportedConnType,
            },
            ServerError::IoError(_) => Error {
                msg: err_msg,
                kind: ErrorKind::IO,
            },
        };
        err.serialize(serializer)
    }
}