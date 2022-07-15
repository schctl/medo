use opencv::core::{Mat, MatTraitConst, MatTraitConstManual, Scalar, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

use medo_stacker::contour;
use medo_stacker::homography;
use medo_stacker::stacker::average::Stacker;
use medo_stacker::star;

fn main() {
    let mut args = std::env::args();
    let src_dir = args.nth(1).unwrap();
    let out_path = args.next().unwrap();
    let entries = std::fs::read_dir(src_dir).unwrap();

    // Filter files
    let mut iter = entries.filter_map(|e| e.ok().filter(|v| v.path().is_file()));

    // Create alignment calculator
    let first = imgcodecs::imread(
        iter.next().unwrap().path().as_os_str().to_str().unwrap(),
        imgcodecs::IMREAD_COLOR,
    )
    .unwrap();
    let first_mask = star::find_contours(&first, Default::default())
        .unwrap()
        .collect();
    let first_mask = contour::create_mask(first.size().unwrap(), first.typ(), &first_mask).unwrap();
    let calculator = homography::Calculator::new(&first_mask).unwrap();

    // Align images
    let mut images = iter
        .map(move |p| {
            // Read image
            let image = imgcodecs::imread(
                p.path().as_os_str().to_str().unwrap(),
                imgcodecs::IMREAD_COLOR,
            )
            .unwrap();

            // Create mask
            let mask = star::find_contours(&image, Default::default())
                .unwrap()
                .collect();
            let mask = contour::create_mask(image.size().unwrap(), image.typ(), &mask).unwrap();

            // Align
            let warp = calculator.calculate(&mask, Default::default()).unwrap();
            let mut dst = Mat::default();
            imgproc::warp_perspective(
                &image,
                &mut dst,
                &warp,
                image.size().unwrap(),
                imgproc::INTER_LINEAR,
                opencv::core::BORDER_CONSTANT,
                Scalar::default(),
            )
            .unwrap();

            dst
        })
        .collect::<Vec<Mat>>();

    // Stack
    images.extend([first]);
    let stacker = Stacker::new(images);
    let res = stacker.last().unwrap();

    // Write result
    println!("{}", out_path);
    imgcodecs::imwrite(&out_path, &res, &Vector::new()).unwrap();
}
