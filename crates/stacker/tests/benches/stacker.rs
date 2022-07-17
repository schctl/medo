use criterion::{criterion_group, criterion_main, Criterion};

use medo_stacker::stacker::average;
use medo_stacker_tests::common;

pub fn average_stack(c: &mut Criterion) {
    // Read test images
    let image = common::read_image("image").unwrap();
    let template = common::read_image("template").unwrap();
    // Run benchmark
    c.bench_function("Basic Average Stacking", |b| {
        b.iter(|| {
            let stacker = average::Stacker::new([image.clone(), template.clone()]);
            let _last = stacker.last().unwrap();
        })
    });
}

criterion_group! {
    name = stacker_benches;
    config = Criterion::default();
    targets = average_stack
}
criterion_main!(stacker_benches);
