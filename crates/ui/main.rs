use std::borrow::Cow;
use std::path::PathBuf;

use medo::core::entry::{Entries, Entry};
use medo::core::util;
use medo::group;
use medo::pipeline;

fn init_log() {
    #[cfg(debug_assertions)]
    let level = tracing::Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let level = tracing::Level::INFO;
    let format = tracing_subscriber::fmt::format()
        .without_time()
        .with_thread_names(true);
    #[cfg(not(debug_assertions))]
    let format = format.with_target(false);
    tracing_subscriber::fmt()
        .event_format(format)
        .with_max_level(level)
        .init();
}

fn main() {
    // Initialization
    init_log();
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    // Parse opts
    let mut args = std::env::args();
    let src_dir = PathBuf::from(&args.nth(1).unwrap());
    let out_path = PathBuf::from(&args.next().unwrap());

    // Run app
    let mut entries = std::fs::read_dir(&src_dir).unwrap().filter_map(|d| {
        d.ok()
            .and_then(|d| Entry::new_path_owned(d.path()).ok().map(Cow::Owned))
    });
    let reference = entries.next().unwrap();
    let entries = Entries { reference, entries }.into_owned();
    // Create default group
    let pipeline = pipeline::Pipeline {
        stages: vec![
            pipeline::Stage::Alignment(Default::default()),
            pipeline::Stage::Sharpen(Default::default()),
            pipeline::Stage::Stacking(Default::default()),
        ],
    };
    let mut group = group::Group {
        name: "default".to_owned(),
        pipeline,
        entries,
        pipeline_output: None,
    };
    let out = group.process().unwrap();

    // Write result
    util::write_image(&out_path, out.reference.read_image().unwrap().as_ref()).unwrap();
}
