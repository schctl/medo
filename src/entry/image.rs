//! Image based entry.

use std::hash::Hash;

use opencv::core::Mat;

use crate::Result;

/// An owned image.
#[derive(Debug, Clone)]
pub struct Image {
    name: String,
    image: Mat,
}

// SAFETY: access to the underlying image will be controlled  so this should be ok.
unsafe impl Send for Image {}
unsafe impl Sync for Image {}

impl PartialEq for Image {
    fn eq(&self, other: &Self) -> bool {
        self.name.eq(&other.name)
    }
}

impl Eq for Image {}

impl Hash for Image {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state)
    }
}

impl Image {
    /// Create a new entry from an image.
    pub fn new<OwnString: ToString>(name: OwnString, image: Mat) -> Result<Self> {
        Self::new_owned(name.to_string(), image)
    }

    /// Create a new entry from an image.
    pub fn new_owned(name: String, image: Mat) -> Result<Self> {
        Ok(Self { name, image })
    }

    /// Get the name associated to this entry.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Get the underlying image.
    pub fn image(&self) -> &Mat {
        &self.image
    }
}
