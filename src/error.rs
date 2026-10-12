use thiserror::Error;

#[derive(Debug, Error)]
pub enum SwampnetError {
    #[error("I/O error occurred")]
    Io(#[from] std::io::Error),
}

pub type SwampnetResult<T> = Result<T, SwampnetError>;
