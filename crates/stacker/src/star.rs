//! Tools to detect stars in an image and create a mask.

use opencv::core::{Mat, Point, Point2f, Vector};
use opencv::imgproc;

use crate::contour::Contour;
use crate::Result;

/// Get the area of a circle.
fn area_of_circle(radius: f32) -> f32 {
    std::f32::consts::PI * radius.powi(2)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct DetectionOpts {
    /// Maximum area that can be filled by a star.
    pub max_area: f32,
    /// Maximum difference in shape from a perfect circle that a star can appear as (percent from [0, 1]).
    pub max_eccentricity: f32,
}

impl Default for DetectionOpts {
    fn default() -> Self {
        Self {
            max_area: 2500.0,
            max_eccentricity: 0.5,
        }
    }
}

/// Check if a contour resembles a star.
pub fn is_contour_a_star(cnt: &Contour, opts: DetectionOpts) -> Result<bool> {
    let area = imgproc::contour_area(&cnt, false)? as f32;

    // Reject if contour is too large
    if area > opts.max_area {
        return Ok(false);
    }

    // Reject if the contour area is similar to the minimum enclosing circle
    let mut radius = 0.0;
    let mut center = Point2f::new(0.0, 0.0);
    imgproc::min_enclosing_circle(&cnt, &mut center, &mut radius)?;

    let circle = area_of_circle(radius);
    if (area - circle).abs() > (opts.max_eccentricity * circle) {
        return Ok(false);
    }

    Ok(true)
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContourDetectionOpts {
    /// Star detections options.
    pub star_detection: DetectionOpts,
    /// Threshold brightness.
    pub threshold_brightness: f32,
    /// Maximum brightness.
    pub max_brightness: f32,
}

impl Default for ContourDetectionOpts {
    fn default() -> Self {
        Self {
            star_detection: Default::default(),
            threshold_brightness: 144.0,
            max_brightness: 255.0,
        }
    }
}

/// Find all star contours from an image.
pub fn find_contours(
    img: &Mat,
    opts: ContourDetectionOpts,
) -> Result<impl Iterator<Item = Contour>> {
    // Convert image to grayscale, blur and threshold
    let mut img_gray = Mat::default();
    imgproc::cvt_color(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY, 0)?;
    // let mut img_blur = Mat::default();
    // imgproc::median_blur(&img_gray, &mut img_blur, 5)?;
    let mut img_thresh = Mat::default();
    imgproc::threshold(
        &img_gray,
        &mut img_thresh,
        opts.threshold_brightness as f64,
        opts.max_brightness as f64,
        imgproc::THRESH_BINARY,
    )?;

    // Find contours
    let mut contours: Vector<Contour> = Vector::new();
    imgproc::find_contours(
        &img_thresh,
        &mut contours,
        imgproc::RETR_TREE,
        imgproc::CHAIN_APPROX_SIMPLE,
        Point::default(),
    )?;

    Ok(contours.into_iter().filter_map(move |s| {
        is_contour_a_star(&s, opts.star_detection)
            .ok()
            .and_then(|o| o.then(|| s))
    }))
}
