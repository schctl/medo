//! Method of stacking by averaging.

use std::borrow::Cow;

use medo_core::cv;
use medo_core::cv::core::Mat;
use medo_core::entry::{self, Entry};
use medo_core::Result;

#[must_use = "iterators are lazy and do nothing unless consumed"]
pub struct Stacker<'iter, T: Iterator<Item = Cow<'iter, Entry>>> {
    out: entry::Image,
    iter: T,
    prog: usize,
}

impl<'iter, T: Iterator<Item = Cow<'iter, Entry>>> Stacker<'iter, T> {
    pub fn new<F: IntoIterator<Item = T::Item, IntoIter = T>>(iter: F) -> Result<Self> {
        let mut iter = iter.into_iter();
        let out = iter.next().unwrap().into_owned().into_image()?;
        Ok(Self { out, iter, prog: 0 })
    }

    /// Leak the underlying data store.
    #[inline]
    pub fn leak(self) -> entry::Image {
        self.out
    }
}

impl<'iter, T: Iterator<Item = Cow<'iter, Entry>>> Iterator for Stacker<'iter, T> {
    type Item = Result<()>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|next| {
            // Calculate weight
            let alpha = 1.0 / (self.prog + 2) as f64;
            let beta = 1.0 - alpha;
            // Add
            let mut new = Mat::default();
            cv::core::add_weighted(
                next.read_image()?.as_ref(),
                alpha,
                self.out.image(),
                beta,
                0.0,
                &mut new,
                -1,
            )?;
            // Update progress
            self.out.replace_image(new)?;
            self.prog += 1;
            Ok(())
        })
    }
}
