use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] std::io::Error),
    #[error(transparent)]
    Gcb(#[from] greycat_gcb::Error),
    #[error("deserializing {0}")]
    Other(#[from] anyhow::Error),
}
