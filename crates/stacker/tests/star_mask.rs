use opencv::prelude::{MatTraitConst, MatTraitConstManual};

use medo_stacker::contour;
use medo_stacker::star;

mod common;

#[test]
fn create_star_mask() {
    // Read test image
    let image = common::read_image("template");
    // Find contours and create mask
    let contours = star::find_contours(&image, Default::default())
        .unwrap()
        .collect();
    let mask = contour::create_mask(image.size().unwrap(), image.typ(), &contours).unwrap();
    // Write results
    common::write_image("star_mask", &mask);
}
