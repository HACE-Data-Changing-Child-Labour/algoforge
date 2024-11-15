use thiserror::Error;

#[allow(dead_code)]
#[derive(Error, Debug)]
pub enum LibError {
    #[error("IO error: {0}")]
    IO(String),

    #[error("Unknown error: {0}")]
    Unknown(String),
}
