//! Tools to detect stars in an image and create a mask.

use opencv::core::{Mat, Point, Point2f, Vector};
use opencv::imgproc;

use crate::Result;

/// Get the area of a circle.
fn area_of_circle(radius: f32) -> f32 {
    std::f32::consts::PI * radius.powi(2)
}

/// Check if a contour resembles a star.
pub fn is_contour_a_star(cnt: &Vector<Point>) -> Result<bool> {
    let area = imgproc::contour_area(&cnt, false)?;

    // Reject if contour is too large
    if area > 2500.0 {
        return Ok(false);
    }

    // Reject if the contour area is similar to the minimum enclosing circle
    let mut radius = 0.0;
    let mut center = Point2f::new(0.0, 0.0);
    imgproc::min_enclosing_circle(&cnt, &mut center, &mut radius)?;

    if (area - area_of_circle(radius) as f64).abs() > (0.5 * area) {
        return Ok(false);
    }

    Ok(true)
}

/// Find all star contours from an image.
pub fn find_contours(img: &Mat) -> Result<Vector<Vector<Point>>> {
    // Convert image to grayscale, blur and threshold
    let mut img_gray = Mat::default();
    imgproc::cvt_color(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY, 0)?;
    // let mut img_blur = Mat::default();
    // imgproc::median_blur(&img_gray, &mut img_blur, 5)?;
    let mut img_thresh = Mat::default();
    imgproc::threshold(
        &img_gray,
        &mut img_thresh,
        127.0,
        255.0,
        imgproc::THRESH_BINARY,
    )?;

    // Find contours
    let mut contours: Vector<Vector<Point>> = Vector::new();
    imgproc::find_contours(
        &img_thresh,
        &mut contours,
        imgproc::RETR_TREE,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::default(),
    )?;

    Ok(contours
        .into_iter()
        .filter_map(|s| is_contour_a_star(&s).ok().and_then(|o| o.then(|| s)))
        .collect())
}
