use criterion::{criterion_group, criterion_main, Criterion};
use medo_core::entry::Entry;

use std::borrow::Cow;

use medo_stacker::stacker::Stacker;
use medo_stacker_tests::common;

pub fn average_stack(c: &mut Criterion) {
    // Read test images
    let image = Entry::new_image(
        "image",
        common::read_image("image").unwrap(),
    ).unwrap();
    let template = Entry::new_image(
        "template",
        common::read_image("template").unwrap(),
    ).unwrap();

    // Run benchmark
    let iter: [Cow<Entry>; 2] = [Cow::Borrowed(&image), Cow::Borrowed(&template)];
    c.bench_function("Basic Average Stacking", |b| {
        b.iter(|| {
            let stacker = Stacker::average(iter.clone()).unwrap();
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
