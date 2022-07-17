//! Image stacking tools.

use std::borrow::Cow;

use medo_core::entry::Entry;
use medo_core::Result;

pub mod average;

/// A wrapper around stacker types.
pub enum Stacker<'iter, T: Iterator<Item = Cow<'iter, Entry>>> {
    Average(average::Stacker<'iter, T>),
}

impl<'iter, T: Iterator<Item = Cow<'iter, Entry>>> Stacker<'iter, T> {
    #[inline]
    pub fn average<F: IntoIterator<Item = T::Item, IntoIter = T>>(iter: F) -> Result<Self> {
        Ok(Self::Average(average::Stacker::new(iter)?))
    }

    /// Leak the underlying data store.
    #[inline]
    pub fn leak(self) -> Entry {
        match self {
            Self::Average(a) => a.leak(),
        }
    }
}

impl<'iter, T: Iterator<Item = Cow<'iter, Entry>>> Iterator for Stacker<'iter, T> {
    type Item = Result<&'iter Entry>;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        match self {
            Self::Average(a) => a.next(),
        }
    }
}
