//! [ECC](Self::Calculator) based homography calculator.

use opencv::core::{Mat, TermCriteria};
use opencv::imgproc;
use opencv::video;

use super::Calculate;
use crate::Result;

pub struct Calculator {
    dst: Mat,
}

impl Calculator {
    pub fn new(dst: &Mat) -> Result<Self> {
        let mut dst_gray = Mat::default();
        imgproc::cvt_color(&dst, &mut dst_gray, imgproc::COLOR_BGR2GRAY, 0)?;
        Ok(Self { dst: dst_gray })
    }
}

impl Calculate for Calculator {
    fn calculate(&mut self, src: &Mat) -> Result<Mat> {
        // Convert image to grayscale
        let mut src_gray = Mat::default();
        imgproc::cvt_color(src, &mut src_gray, imgproc::COLOR_BGR2GRAY, 0)?;

        // Find warp matrix

        // Define the termination criteria
        let criteria = TermCriteria {
            typ: opencv::core::TermCriteria_Type::COUNT as i32
                | opencv::core::TermCriteria_Type::EPS as i32,
            max_count: 100,
            epsilon: 1e-10,
        };

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
