use std::ffi::OsStr;
use std::io;
use std::path::PathBuf;

use crate::{Error, Result};

/// A single image entry.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entry {
    path: PathBuf,
}

impl Entry {
    /// Create a new image entry from a path.
    pub fn new<PathRef, OwnPath>(path: PathRef) -> Result<Self>
    where
        OwnPath: ToOwned<Owned = PathBuf>,
        PathRef: AsRef<OwnPath>,
    {
        Self::new_owned(path.as_ref().to_owned())
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
    pub const fn path(&self) -> &PathBuf {
        &self.path
    }

    /// Get the name of this entry.
    pub fn file_name(&self) -> &OsStr {
        self.path.file_name().unwrap()
    }
}

/// A group of entries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entries {
    /// The primary entry in this group of entries.
    pub reference: Entry,
    pub entries: Vec<Entry>,
}

impl Entries {
    /// Create a new set of entries.
    pub fn new(entries: impl IntoIterator<Item = Entry>) -> Result<Self> {
        let mut entries: Vec<_> = entries.into_iter().collect();

        if entries.len() < 1 {
            return Err(Error::OtherStatic("no entries provided"));
        }

        let reference = entries.remove(0);

        Ok(Self { reference, entries })
    }
}
