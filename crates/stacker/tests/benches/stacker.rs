use criterion::{criterion_group, criterion_main, Criterion};

use std::borrow::Cow;

use medo_core::entry::Entry;
use medo_stacker::stacker::Stacker;
use medo_stacker_tests::common;

pub fn stack_average_basic(c: &mut Criterion) {
    // Read test images
    let image = Entry::new_image("image", common::read_image("image.jpg").unwrap()).unwrap();
    let template =
        Entry::new_image("template", common::read_image("template.jpg").unwrap()).unwrap();

    // Run benchmark
    let iter: [Cow<Entry>; 2] = [Cow::Borrowed(&image), Cow::Borrowed(&template)];
    c.bench_function("Basic Average Stacking", |b| {
        b.iter(|| {
            let stacker = Stacker::average(iter.clone()).unwrap();
            let _last = stacker.last().unwrap().unwrap();
        })
    });
}

criterion_group! {
    name = stacker_benches;
    config = Criterion::default();
    targets = stack_average_basic
}
criterion_main!(stacker_benches);
