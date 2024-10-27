use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter};

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Error {
    pub msg: String,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub enum ErrorKind {
    Unknown,
    Redis,
    UnsupportedConnType,
    IO,
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, {}", self.kind, self.msg)
    }
}

impl std::error::Error for Error {}