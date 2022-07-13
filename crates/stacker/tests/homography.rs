use std::path::Path;

use opencv::core::{Mat, ToInputArray, Vector};
use opencv::imgcodecs;
use opencv::imgproc;
use opencv::prelude::_InputArrayTraitConst;

use medo_stacker::homography::Calculator;

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
fn warp_with_homography() {
    // Read test images
    let image =
        imgcodecs::imread(&relative("tests/data/image.jpg"), imgcodecs::IMREAD_COLOR).unwrap();
    let template = imgcodecs::imread(
        &relative("tests/data/template.jpg"),
        imgcodecs::IMREAD_COLOR,
    )
    .unwrap();

    // Calculate homography
    let mut calculator = Calculator::new(&template).unwrap();
    let homography = calculator.calculate(&image).unwrap();

    // Warp image using homography
    let mut dst = Mat::default();
    imgproc::warp_perspective(
        &image,
        &mut dst,
        &homography,
        template.input_array().unwrap().size(-1).unwrap(),
        imgproc::INTER_LINEAR,
        opencv::core::BORDER_CONSTANT,
        opencv::core::Scalar::default(),
    )
    .unwrap();

    // Write result
    imgcodecs::imwrite(
        &relative_target(format!("ecc.jpg")),
        &dst,
        &Vector::new(),
    )
    .unwrap();
}
