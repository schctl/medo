//! Defines image entries.

use std::borrow::Cow;
use std::hash::Hash;
use std::path::PathBuf;

use opencv::core::Mat;

use crate::{util, Result};

mod image;
mod path;
pub use image::Image;
pub use path::Path;

/// An entry represents an image.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Entry {
    Path(Path),
    Image(Image),
}

// Constructors
impl Entry {
    /// Create a new path-based entry.
    #[inline]
    pub fn new_path<OwnPath>(path: OwnPath) -> Result<Self>
    where
        OwnPath: ToOwned<Owned = PathBuf>,
    {
        Ok(Self::Path(Path::new(path)?))
    }

    /// Create a new path-based entry from an owned path.
    #[inline]
    pub fn new_path_owned(path: PathBuf) -> Result<Self> {
        Ok(Self::Path(Path::new_owned(path)?))
    }

    /// Create a new image-based entry.
    #[inline]
    pub fn new_image<OwnString: ToString>(name: OwnString, image: Mat) -> Result<Self> {
        Ok(Self::Image(Image::new(name, image)?))
    }

    /// Create a new image-based entry.
    #[inline]
    pub fn new_image_owned(name: String, image: Mat) -> Result<Self> {
        Ok(Self::Image(Image::new_owned(name, image)?))
    }
}

impl Entry {
    /// Get the name associated to this entry.
    #[inline]
    pub fn name(&self) -> Cow<'_, str> {
        match self {
            Self::Path(p) => p.file_name(),
            Self::Image(p) => Cow::Borrowed(p.name()),
        }
    }

    /// Get the image associated with this entry.
    #[inline]
    pub fn read_image(&self) -> Result<Cow<'_, Mat>> {
        match self {
            Self::Path(p) => Ok(Cow::Owned(util::read_image(p.path())?)),
            Self::Image(p) => Ok(Cow::Borrowed(p.image())),
        }
    }

    /// Get and *own* the image associated with this entry.
    #[inline]
    pub fn read_into_image(&mut self) -> Result<&Mat> {
        match self {
            Self::Path(p) => {
                *self = Self::new_image(p.file_name().as_ref(), util::read_image(p.path())?)?;
                self.read_into_image()
            }
            Self::Image(p) => Ok(p.image()),
        }
    }
}

/// A group of entries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Entries<'entry, EntryIter: Iterator<Item = Cow<'entry, Entry>>> {
    pub reference: Cow<'entry, Entry>,
    pub entries: EntryIter,
}

impl<'entry, EntryIter: Iterator<Item = Cow<'entry, Entry>>> Entries<'entry, EntryIter> {
    #[inline]
    pub fn into_owned(self) -> OwnedEntries {
        OwnedEntries {
            reference: self.reference.into_owned(),
            entries: self.entries.map(|e| e.into_owned()).collect(),
        }
    }
}

/// A generic owned iterator over entry references.
///
/// Defined as a convenient shorthand.
pub type OwnedEntryIter<'entry> =
    Box<dyn Iterator<Item = Cow<'entry, Entry>> + Send + Sync + 'entry>;

/// An owned group of entries.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OwnedEntries {
    /// The primary entry in this group of entries.
    pub reference: Entry,
    pub entries: Vec<Entry>,
}

impl OwnedEntries {
    /// Return a borrowed group of entries.
    #[inline]
    pub fn to_borrow(&self) -> Entries<OwnedEntryIter> {
        Entries {
            reference: Cow::Borrowed(&self.reference),
            entries: Box::new(self.entries.iter().map(Cow::Borrowed)),
        }
    }
}
