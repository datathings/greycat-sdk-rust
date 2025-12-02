mod error;
mod reader;

pub use error::*;
pub use reader::*;
pub type Result<T> = std::result::Result<T, Error>;
