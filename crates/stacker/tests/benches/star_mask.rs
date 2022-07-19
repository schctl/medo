use criterion::{black_box, criterion_group, criterion_main, Criterion};

use medo_stacker::star;
use medo_stacker_tests::common;

pub fn star_detection(c: &mut Criterion) {
    // Read test image
    let image = common::read_image("image.jpg").unwrap();
    // Run benchmark
    c.bench_function("Basic Star Detection", |b| {
        b.iter(|| {
            let _contours: Vec<_> = black_box(
                star::find_contours(&image, Default::default())
                    .unwrap()
                    .collect(),
            );
        })
    });
}

criterion_group! {
    name = star_mask_benches;
    config = Criterion::default();
    targets = star_detection
}
criterion_main!(star_mask_benches);
