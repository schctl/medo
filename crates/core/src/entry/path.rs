//! Path based image entry.

use std::borrow::Cow;
use std::io;
use std::path::{Path as PathRef, PathBuf};

use crate::Result;

/// A path to an image.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Path {
    path: PathBuf,
}

impl Path {
    /// Create a new image entry from a path.
    #[inline]
    pub fn new<OwnPath>(path: OwnPath) -> Result<Self>
    where
        OwnPath: ToOwned<Owned = PathBuf>,
    {
        Self::new_owned(path.to_owned())
    }

    /// Create a new image entry from a path.
    pub fn new_owned(path: PathBuf) -> Result<Self> {
        if !path.exists() || path.is_dir() || path.file_name().is_none() {
            // FIXME: use right errors when `io_error_more` is stabilized
            return Err(io::ErrorKind::NotFound.into());
        }
        Ok(Self { path })
    }

    /// Get the path to this entry.
    #[inline]
    pub fn path(&self) -> &PathRef {
        self.path.as_path()
    }

    /// Get the name of this entry.
    #[inline]
    pub fn file_name(&self) -> Cow<'_, str> {
        self.path.file_name().unwrap().to_string_lossy()
    }
}
