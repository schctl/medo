use std::path::Path;

use opencv::core::Vector;
use opencv::imgcodecs;
use opencv::prelude::{MatTraitConst, MatTraitConstManual};

use medo_stacker::contour;
use medo_stacker::star;

fn relative<P: AsRef<Path>>(path: P) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path.as_ref().display())
}

fn relative_target<P: AsRef<Path>>(path: P) -> String {
    format!(
        "{}/{}",
        env!("CARGO_TARGET_TMPDIR"),
        path.as_ref().display()
    )
}

#[test]
fn create_mask() {
    // Read test image
    let image = imgcodecs::imread(
        &relative("tests/data/template.jpg"),
        imgcodecs::IMREAD_COLOR,
    )
    .unwrap();

    // Find contours and create mask
    let contours = star::find_contours(&image).unwrap();
    let mask = contour::create_mask(image.size().unwrap(), image.typ(), &contours).unwrap();

    // Write result
    imgcodecs::imwrite(
        &relative_target(format!("star_mask.jpg")),
        &mask,
        &Vector::new(),
    )
    .unwrap();
}
