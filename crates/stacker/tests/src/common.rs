use std::path::Path;

use medo_core::cv::core::Mat;
use medo_core::util;
use medo_core::Result;

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

pub fn read_image(name: &str) -> Result<Mat> {
    util::read_image(relative(format!("tests/data/{}.jpg", name)))
}

pub fn write_image(name: &str, image: &Mat) -> Result<()> {
    util::write_image(relative_target(format!("{}.jpg", name)), image)
}
