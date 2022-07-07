//! Tools to find the relation between projections of an image.
//!
//! A **homography** is a mapping between two planar projections of an image. A **homography matrix**
//! is a transformation that describes the homography between images. This module provides tools
//! to compute the homography matrix of an image based on a target image.

pub mod ecc;
pub mod orb;

use enum_dispatch::enum_dispatch;
use opencv::core::Mat;

use crate::Result;

/// The method for calculating the homography matrix of two images.
#[derive(Debug, Clone, Copy, Hash)]
pub enum Method {
    /// Calculation based on the [ORB] keypoint detection algorithm.
    ///
    /// [orb]: self::orb::Calculator
    Orb,
    /// Calculation based on the [ECC] image alignment algorithm.
    ///
    /// [ecc]: self::ecc::Calculator
    Ecc,
}

/// Represents an implementor of a homography calculator.
#[enum_dispatch]
pub trait Calculate {
    fn calculate(&mut self, src: &Mat) -> Result<Mat>;
}

/// Abstraction over different calculator types.
#[enum_dispatch(Calculate)]
pub enum Calculator {
    Orb(orb::Calculator),
    Ecc(ecc::Calculator),
}

impl Calculator {
    pub fn new(dst: &Mat, method: Method) -> Result<Self> {
        match method {
            Method::Orb => Ok(Self::Orb(orb::Calculator::new(dst)?)),
            Method::Ecc => todo!(),
        }
    }
}
