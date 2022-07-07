//! [ECC](Self::Calculator) based homography calculator.

use opencv::core::Mat;

use super::Calculate;
use crate::Result;

pub struct Calculator {}

impl Calculate for Calculator {
    fn calculate(&mut self, _: &Mat) -> Result<Mat> {
        panic!()
    }
}
