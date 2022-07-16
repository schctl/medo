use std::path::PathBuf;

pub mod entry;
pub mod error;
pub mod group;
pub mod medo;
pub mod pipeline;
pub mod util;

pub use entry::{Entries, Entry};
pub use error::*;
pub use group::Group;
pub use medo::Medo;
pub use pipeline::*;

fn init_log() {
    #[cfg(debug_assertions)]
    let level = tracing::Level::DEBUG;
    #[cfg(not(debug_assertions))]
    let level = tracing::Level::INFO;
    let format = tracing_subscriber::fmt::format()
        .without_time()
        .with_thread_names(true);
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
    let entries = std::fs::read_dir(&src_dir)
        .unwrap()
        .filter_map(|d| d.ok().map(|d| Entry::new_owned(d.path()).ok()).flatten());
    let entries = Entries::new(entries).unwrap();
    let pipeline = vec![PipelineStage::Alignment, PipelineStage::Stacking];
    let group = Group {
        name: "default".to_owned(),
        entries,
        pipeline,
    };
    let out = group.process().unwrap();

    // Write result
    util::write_image(&out_path, &out).unwrap();
}
