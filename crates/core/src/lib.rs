//! Core types that are common to all `medo_` crates.

pub mod entry;
pub mod error;
pub mod util;

pub use error::*;
pub use opencv as cv;
