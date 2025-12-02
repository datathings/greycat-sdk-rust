use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("invalid char bytes: {0:?}")]
    InvalidChar([u8; 4]),
    #[error("varint too big (expected at most {0} bytes)")]
    Varint(u8),
}
