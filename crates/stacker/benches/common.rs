use std::path::Path;

use opencv::core::Mat;
use opencv::imgcodecs;

fn relative<P: AsRef<Path>>(path: P) -> String {
    format!("{}/{}", env!("CARGO_MANIFEST_DIR"), path.as_ref().display())
}

pub fn read_image(name: &str) -> Mat {
    imgcodecs::imread(
        &relative(format!("tests/data/{}.jpg", name)),
        imgcodecs::IMREAD_COLOR,
    )
    .unwrap()
}
