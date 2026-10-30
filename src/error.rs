use std::io;
use thiserror::Error;

pub type SwampnetResult<T> = Result<T, SwampnetError>;

#[derive(Debug, Error)]
pub enum SwampnetError {
    #[error("IO Error: {0}")]
    IOError(#[from] io::Error),
}
