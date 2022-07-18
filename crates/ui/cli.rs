//! Command line argument parser.

use clap::Parser;
use std::path::PathBuf;

/// Command line options.
#[derive(Debug, Parser)]
#[clap(name = "medo", version)]
pub struct Opts {
    /// Input directory.
    #[clap(parse(from_os_str))]
    pub input: PathBuf,
    /// Output file.
    #[clap(parse(from_os_str))]
    pub output: PathBuf,
    /// Maximum threads for each unit of work.
    #[clap(short, long, default_value = "4")]
    pub max_threads: usize,
}
