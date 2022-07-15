use std::path::Path;

use opencv::core::{Mat, Vector};
use opencv::imgcodecs;

fn relative<P: AsRef<Path>>(path: P) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path.as_ref().display())
}

fn relative_target<P: AsRef<Path>>(path: P) -> String {
    // I don't like doing this
    std::fs::create_dir_all(format!("{}/target/tmp/", env!("CARGO_MANIFEST_DIR"),)).unwrap();
    format!(
        "{}/target/tmp/{}",
        env!("CARGO_MANIFEST_DIR"),
        path.as_ref().display()
    )
}

pub fn read_image(name: &str) -> Mat {
    imgcodecs::imread(
        &relative(format!("tests/data/{}.jpg", name)),
        imgcodecs::IMREAD_COLOR,
    )
    .unwrap()
}

pub fn write_image(name: &str, image: &Mat) {
    imgcodecs::imwrite(
        &relative_target(format!("{}.jpg", name)),
        image,
        &Vector::new(),
    )
    .unwrap();
}
