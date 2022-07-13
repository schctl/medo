//! Tools to find the relation between projections of an image.
//!
//! A **homography** is a mapping between two planar projections of an image. A **homography matrix**
//! is a transformation that describes the homography between images. This module provides tools
//! to compute the homography matrix of an image based on a target image.

use opencv::core::{Mat, TermCriteria};
use opencv::imgproc;
use opencv::video;

use crate::Result;

/// [ECC] based homography calculator.
///
/// [ecc]: https://sites.google.com/site/georgeevangelidis/ecc
pub struct Calculator {
    dst: Mat,
}

impl Calculator {
    pub fn new(dst: &Mat) -> Result<Self> {
        let mut dst_gray = Mat::default();
        imgproc::cvt_color(&dst, &mut dst_gray, imgproc::COLOR_BGR2GRAY, 0)?;
        Ok(Self { dst: dst_gray })
    }

    pub fn calculate(&mut self, src: &Mat) -> Result<Mat> {
        // Convert image to grayscale
        let mut src_gray = Mat::default();
        imgproc::cvt_color(src, &mut src_gray, imgproc::COLOR_BGR2GRAY, 0)?;

        // Define the termination criteria
        let criteria = TermCriteria {
            typ: opencv::core::TermCriteria_Type::COUNT as i32
                | opencv::core::TermCriteria_Type::EPS as i32,
            max_count: 100,
            epsilon: 1e-10,
        };

        // Calculate warp matrix
        let mut homography = Mat::default();
        video::find_transform_ecc(
            &src_gray,
            &self.dst,
            &mut homography,
            // account for 3D effects
            video::MOTION_HOMOGRAPHY,
            criteria,
            &Mat::default(),
            5,
        )?;

        Ok(homography)
    }
}
