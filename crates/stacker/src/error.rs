//! Our error types.

/// A general error that covers all possible errors.
///
/// This is the error type that should be exposed from all APIs in this crate.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    OpenCV(#[from] opencv::Error),
}

/// Shorthand result type.
pub type Result<T> = ::core::result::Result<T, Error>;
