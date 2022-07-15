use opencv::core::{Mat, Size};
use opencv::imgproc;
use opencv::prelude::{MatExprTraitConst, MatTraitConst, MatTraitConstManual};

use medo_stacker::contour;
use medo_stacker::homography;
use medo_stacker::star;
use medo_stacker_tests::common;

fn warp_image(image: &Mat, warp: &Mat, size: Size) -> Mat {
    let mut dst = Mat::default();
    imgproc::warp_perspective(
        &image,
        &mut dst,
        &warp,
        size,
        imgproc::INTER_LINEAR,
        opencv::core::BORDER_CONSTANT,
        opencv::core::Scalar::default(),
    )
    .unwrap();
    dst
}

fn star_mask_image(img: &Mat) -> Mat {
    // Find contours and create mask
    let contours = star::find_contours(&img, Default::default())
        .unwrap()
        .collect();
    let mask = contour::create_mask(img.size().unwrap(), img.typ(), &contours).unwrap();
    // Apply the mask
    let mut dst = Mat::default();
    opencv::core::bitwise_and(img, &mask, &mut dst, &opencv::core::no_array()).unwrap();
    dst
}

#[test]
fn identical_images_have_no_homography() {
    // Read test image
    let image = common::read_image("image");
    // Calculate homography
    let calculator = homography::Calculator::new(&image).unwrap();
    let homography = calculator.calculate(&image, Default::default()).unwrap();
    // Check result
    let imat = Mat::eye(3, 3, opencv::core::CV_32F)
        .unwrap()
        .to_mat()
        .unwrap();
    let res = (homography - imat).into_result().unwrap().to_mat().unwrap();
    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(
                f32::trunc(*res.at_nd::<f32>(&[i, j]).unwrap() * 100_000.0) / 100_000.0,
                0.0
            );
        }
    }
}

#[test]
fn find_homography_and_warp() {
    // Read test images
    let image = common::read_image("image");
    let template = common::read_image("template");
    // Calculate homography
    let calculator = homography::Calculator::new(&template).unwrap();
    let homography = calculator.calculate(&image, Default::default()).unwrap();
    // Warp image using homography
    let warped = warp_image(&image, &homography, template.size().unwrap());
    // Write result
    common::write_image("ecc", &warped);
}

#[test]
fn find_homography_from_star_mask_and_warp() {
    // Read test images
    let image = common::read_image("image");
    let template = common::read_image("template");
    // Mask images
    let image = star_mask_image(&image);
    let template = star_mask_image(&template);
    // Calculate homography
    let calculator = homography::Calculator::new(&template).unwrap();
    let homography = calculator.calculate(&image, Default::default()).unwrap();
    // Warp image using homography
    let warped = warp_image(&image, &homography, template.size().unwrap());
    // Write result
    common::write_image("ecc_star_mask", &warped);
}