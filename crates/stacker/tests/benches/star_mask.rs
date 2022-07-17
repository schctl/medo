use criterion::{criterion_group, criterion_main, Criterion};

use medo_stacker::contour;
use medo_stacker::star;
use medo_stacker_tests::common;

pub fn find_contours(c: &mut Criterion) {
    // Read test image
    let image = common::read_image("image").unwrap();
    // Run benchmark
    c.bench_function("Basic Star Contour Detection", |b| {
        b.iter(|| {
            let _contours: Vec<_> = star::find_contours(&image, Default::default())
                .unwrap()
                .collect();
        })
    });
}

pub fn star_detection(c: &mut Criterion) {
    // Read test image
    let image = common::read_image("image").unwrap();
    // Run benchmark
    let contour = star::find_contours(&image, Default::default())
        .unwrap()
        .next()
        .unwrap();
    c.bench_function("Basic Star Detection", |b| {
        b.iter(|| {
            let _is_star = star::is_contour_a_star(&contour, Default::default()).unwrap();
        })
    });
}

pub fn draw_mask(c: &mut Criterion) {
    use medo_core::cv::prelude::MatTraitConst;
    use medo_core::cv::prelude::MatTraitConstManual;

    // Read test image
    let image = common::read_image("image").unwrap();
    // Run benchmark
    let contours = star::find_contours(&image, Default::default())
        .unwrap()
        .collect();
    c.bench_function("Basic Star Mask Creation", |b| {
        b.iter(|| {
            let _mask =
                contour::create_mask(image.size().unwrap(), image.typ(), &contours).unwrap();
        })
    });
}

criterion_group! {
    name = star_mask_benches;
    config = Criterion::default();
    targets = find_contours, star_detection, draw_mask
}
criterion_main!(star_mask_benches);
