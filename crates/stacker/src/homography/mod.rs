//! Tools to find the relation between projections of an image.
//!
//! A **homography** is a mapping between two planar projections of an image. A **homography matrix**
//! is a transformation that describes the homography between images. This module provides tools
//! to compute the homography matrix of an image based on a target image.

pub mod ecc;
pub mod orb;

use enum_dispatch::enum_dispatch;
use opencv::core::Mat;

/// The method for calculating the homography matrix of two images.
#[derive(Debug, Clone, Copy, Hash)]
pub enum Method {
    /// Calculation based on the [ORB] keypoint detection algorithm.
    ///
    /// Calculates the homography matrix by computing the distances between keypoints (image features)
    /// of two images. This method is generally faster than [ECC](Self::Ecc), but less precise.
    ///
    /// [orb]: https://en.wikipedia.org/wiki/Oriented_FAST_and_rotated_BRIEF
    Orb,
    /// Calculation based on the [ECC] image alignment algorithm.
    ///
    /// [ecc]: https://sites.google.com/site/georgeevangelidis/ecc
    Ecc,
}

/// Represents an implementor of a homography calculator.
#[enum_dispatch]
pub trait Calculate {
    fn calculate(&mut self, src: Mat) -> Mat;
}

/// Abstraction over different calculator types.
#[enum_dispatch(Calculate)]
pub enum Calculator {
    Orb(orb::Calculator),
    Ecc(ecc::Calculator),
}
