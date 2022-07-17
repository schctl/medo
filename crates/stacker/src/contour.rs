//! Contour tools.

use medo_core::cv;
use medo_core::cv::core::{Mat, Point, Scalar, Size, Vector};
use medo_core::cv::imgproc;
use medo_core::Result;

pub type Contour = Vector<Point>;

/// Create an image mask from the given contours.
pub fn create_mask(size: Size, ty: i32, contours: &Vector<Contour>) -> Result<Mat> {
    let mut mask = Mat::new_size_with_default(size, ty, Scalar::new(0.0, 0.0, 0.0, 0.0))?;

    for i in 0..contours.len() {
        imgproc::draw_contours(
            &mut mask,
            &contours,
            i as i32,
            Scalar::new(255.0, 255.0, 255.0, 255.0),
            -1,
            imgproc::LINE_8,
            &cv::core::no_array(),
            i32::MAX,
            Point::default(),
        )?;
    }

    Ok(mask)
}
