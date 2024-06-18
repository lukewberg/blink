use std::string::FromUtf8Error;
use thiserror::Error;
#[derive(Error, Debug)]
pub enum SerdeError {
    #[error("Failed to deserialize String!")]
    StringFromUtf8(#[from] FromUtf8Error)
}