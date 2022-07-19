//! Tools to detect stars in an image and create a mask.

use medo_core::cv::core::{Mat, Point, Point_, Scalar, Size, Vector};
use medo_core::cv::imgproc;
use medo_core::Result;

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
            max_eccentricity: 0.4,
        }
    }
}

/// A circle used to describe a star in an image.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Circle {
    pub radius: f32,
    pub center: Point_<f32>,
}

impl Circle {
    pub fn area(&self) -> f32 {
        std::f32::consts::PI * self.radius.powi(2)
    }
}

type Contour = Vector<Point_<i32>>;

/// Check if a contour resembles a star.
pub fn map_contour(cnt: &Contour, opts: DetectionOpts) -> Result<Option<Circle>> {
    let area = imgproc::contour_area(&cnt, false)? as f32;

    // Reject if contour is too large
    if area > opts.max_area {
        return Ok(None);
    }

    // Reject if the contour area is not similar to the minimum enclosing circle
    let mut radius = 0.0;
    let mut center = Point_::new(0.0, 0.0);
    imgproc::min_enclosing_circle(&cnt, &mut center, &mut radius)?;
    let circle = Circle { radius, center };

    let circle_area = circle.area();
    if (area - circle_area).abs() > (opts.max_eccentricity * circle_area) {
        return Ok(None);
    }

    Ok(Some(circle))
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct ContourDetectionOpts {
    /// Star detections options.
    pub star_detection: DetectionOpts,
    /// Threshold brightness.
    pub threshold_brightness: f32,
    /// Maximum brightness.
    pub max_brightness: f32,
    /// Blur to apply before masking.
    pub blur_amount: i32,
}

impl Default for ContourDetectionOpts {
    fn default() -> Self {
        Self {
            star_detection: Default::default(),
            threshold_brightness: 144.0,
            max_brightness: 255.0,
            blur_amount: 3,
        }
    }
}

/// Find all star contours from an image.
pub fn find_contours(
    img: &Mat,
    opts: ContourDetectionOpts,
) -> Result<impl Iterator<Item = Circle>> {
    // Convert image to grayscale, blur and threshold
    let mut img_gray = Mat::default();
    imgproc::cvt_color(&img, &mut img_gray, imgproc::COLOR_BGR2GRAY, 0)?;
    let mut img_blur = Mat::default();
    imgproc::median_blur(&img_gray, &mut img_blur, opts.blur_amount)?;
    let mut img_thresh = Mat::default();
    imgproc::threshold(
        &img_blur,
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

    Ok(contours
        .into_iter()
        .filter_map(move |s| map_contour(&s, opts.star_detection).ok().flatten()))
}

/// Create an image mask from the given stars.
pub fn create_mask(size: Size, ty: i32, stars: impl Iterator<Item = Circle>) -> Result<Mat> {
    let mut mask = Mat::new_size_with_default(size, ty, Scalar::new(0.0, 0.0, 0.0, 0.0))?;

    for star in stars {
        let center = Point_ {
            x: star.center.x as i32,
            y: star.center.y as i32,
        };
        let radius = star.radius.ceil() as i32;

        imgproc::circle(
            &mut mask,
            center,
            radius,
            Scalar::new(255.0, 255.0, 255.0, 255.0),
            -1,
            imgproc::LINE_8,
            0,
        )?;
    }

    Ok(mask)
}
