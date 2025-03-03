use std::string::FromUtf8Error;
use thiserror::Error;

use super::VarIntError;
#[derive(Error, Debug)]
pub enum SerdeError {
    #[error("Failed to deserialize String!")]
    StringFromUtf8(#[from] FromUtf8Error),
    #[error("Failed to parse varint from packet!")]
    VarInt(#[from] VarIntError),
    #[error("")]
    Io(#[from] std::io::Error),
}

#[derive(Error, Debug)]
pub enum HandlerError {
    #[error("")]
    Serde(#[from] SerdeError),
    #[error("")]
    Io(#[from] std::io::Error),
}