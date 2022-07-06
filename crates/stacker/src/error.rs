#[derive(Debug, thiserror::Error)]
pub enum Error {
    #[error(transparent)]
    OpenCV(#[from] opencv::Error),
}

pub type Result<T> = ::core::result::Result<T, Error>;
