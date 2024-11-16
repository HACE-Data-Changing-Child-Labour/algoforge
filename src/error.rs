use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum LibError {
    #[error("IO error: {0}")]
    IO(String),

    #[error("Invalid input for processor: {0} - expected type: {1}")]
    InvalidInput(String, String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
