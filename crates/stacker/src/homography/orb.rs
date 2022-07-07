//! [ORB](Self::Calculator) based homography calculator.

use std::cmp::Ordering;

use opencv::calib3d;
use opencv::core::{self as opencv_core, KeyPoint, Mat, Point2f, Ptr, Vector};
use opencv::features2d::BFMatcher;
use opencv::prelude::{DescriptorMatcher, Feature2DTrait, ORB};

use super::Calculate;
use crate::Result;

#[derive(Debug, Clone)]
struct Descriptor {
    kp: Vector<KeyPoint>,
    desc: Mat,
}

impl Descriptor {
    fn from_src(orb: &mut Ptr<dyn ORB>, src: &Mat) -> Result<Self> {
        let mut kp = Vector::new();
        let mut desc = Mat::default();
        orb.detect_and_compute(src, &Mat::default(), &mut kp, &mut desc, false)?;
        Ok(Self { kp, desc })
    }
}

/// [ORB] based keypoint difference calculator.
///
/// Calculates the homography matrix by computing the distances between keypoints (image features)
/// of two images. This method is generally faster than [ECC](self::ecc::Calculator), but less precise.
///
/// [orb]: <https://en.wikipedia.org/wiki/Oriented_FAST_and_rotated_BRIEF>
pub struct Calculator {
    orb: Ptr<dyn ORB>,
    matcher: Ptr<BFMatcher>,
    dst: Descriptor,
}

impl Calculator {
    pub fn new(dst: &Mat) -> Result<Self> {
        let mut orb = <dyn ORB>::default()?;
        let dst = Descriptor::from_src(&mut orb, dst)?;
        let matcher = BFMatcher::create(opencv_core::NORM_HAMMING, true)?;
        Ok(Self { orb, matcher, dst })
    }
}

impl Calculate for Calculator {
    fn calculate(&mut self, src: &Mat) -> Result<Mat> {
        let src = Descriptor::from_src(&mut self.orb, src)?;
        let mut matches = Vector::new();

        // Run matcher
        self.matcher.add(&src.desc)?;
        self.matcher
            .match_(&self.dst.desc, &mut matches, &Mat::default())?;
        self.matcher.clear()?;

        // Sort matches
        let mut matches = matches.to_vec();
        matches.sort_by(|a, b| {
            a.distance
                .partial_cmp(&b.distance)
                .map_or(Ordering::Equal, |t| t)
        });

        // Define mapping between source and dest keypoints
        let mut src_kp: Vector<Point2f> = Vector::with_capacity(matches.len());
        let mut dst_kp: Vector<Point2f> = Vector::with_capacity(matches.len());

        for m in matches {
            src_kp.push(self.dst.kp.get(m.query_idx as usize)?.pt);
            dst_kp.push(src.kp.get(m.train_idx as usize)?.pt);
        }

        // Calculate homography matrix
        Ok(calib3d::find_homography(
            &dst_kp,
            &src_kp,
            &mut Mat::default(),
            calib3d::RANSAC,
            3.0,
        )?)
    }
}
