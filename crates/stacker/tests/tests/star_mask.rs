use medo_core::cv::prelude::{MatTraitConst, MatTraitConstManual};
use medo_stacker;
use medo_stacker::star;
use medo_stacker_tests::common;

#[test]
fn create_star_mask() {
    // Read test image
    let image = common::read_image("template").unwrap();
    // Find contours and create mask
    let contours = star::find_contours(&image, Default::default()).unwrap();
    let mask = star::create_mask(image.size().unwrap(), image.typ(), contours).unwrap();
    // Write results
    common::write_image("star_mask", &mask).unwrap();
}
