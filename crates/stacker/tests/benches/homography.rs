use criterion::{criterion_group, criterion_main, Criterion};

use medo_stacker::homography;
use medo_stacker_tests::common;

pub fn basic_homography(c: &mut Criterion) {
    // Read test images
    let image = common::read_image("image").unwrap();
    let template = common::read_image("template").unwrap();
    let calculator = homography::Calculator::new(&template).unwrap();
    // Run benchmark
    c.bench_function("Basic Alignment Calculation", |b| {
        b.iter(|| {
            calculator
                .calculate(&image, homography::CalculateOpts { iterations: 100 })
                .unwrap()
        })
    });
}

fn short_sample_size() -> Criterion {
    Criterion::default().sample_size(10)
}

criterion_group! {
    name = homography_benches;
    config = short_sample_size();
    targets = basic_homography
}
criterion_main!(homography_benches);
