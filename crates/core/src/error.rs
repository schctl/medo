//! Our error types.

use std::io;

/// A general error that covers all possible errors.
///
/// This is the error type that should be exposed from all APIs in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    Io(#[from] io::Error),
    #[error(transparent)]
    OpenCV(#[from] opencv::Error),
    // FIXME: provide information on the stage
    #[error("pipeline stage failed")]
    PipelineStageFailed,
    #[error("{0}")]
    Other(String),
    #[error("{0}")]
    OtherStatic(&'static str),
}

impl From<io::ErrorKind> for Error {
    #[inline]
    fn from(e: io::ErrorKind) -> Self {
        Self::Io(io::Error::from(e))
    }
}

/// Shorthand result type.
pub type Result<T> = ::core::result::Result<T, Error>;
