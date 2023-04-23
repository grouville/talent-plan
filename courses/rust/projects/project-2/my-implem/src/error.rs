use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum MyError {
    #[error("Key not found")]
    KeyNotFound,
    #[error("I/O error: {0}")]
    Io(#[from] io::Error),
    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),
}

pub type Result<T> = std::result::Result<T, MyError>;
