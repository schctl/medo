//! Image stacking tools.

use medo_core::cv::core::Mat;
use medo_core::Result;

pub mod average;

/// A marker trait describing the behavior of a stacking iterator.
///
/// The behavior expected of a stacker is to stack each subsequent provided image on each iteration and
/// returning its progress. Once the stacking is over, return [`None`].
pub trait Stacker: Iterator<Item = Result<Mat>> {}
