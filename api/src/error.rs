use serde::{Deserializer, Serializer};
use std::fmt::{Display, Formatter};

#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Redis(#[from] redis::RedisError),
    #[error(transparent)]
    IO(#[from] std::io::Error),
    NonsupportConnType,
}

impl serde::Serialize for Error {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_str(self.to_string().as_str())
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::Redis(re) => write!(f, "Redis error: {}", re),
            Error::IO(_) => write!(f, "IO error"),
            Error::NonsupportConnType => write!(f, "Nonsupport connection type"),
        }
    }
}