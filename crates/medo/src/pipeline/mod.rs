//! Defines an entry group's processing pipeline.

use medo_core::entry::{Entries, OwnedEntryIter};
use medo_core::Result;

pub mod alignment;
pub mod sharpen;
pub mod stacking;

/// A stage in the processing pipeline of a group of entries.
#[derive(Debug, Clone)]
pub enum Stage {
    Alignment(alignment::Opts),
    Stacking(stacking::Opts),
    Sharpen(sharpen::Opts),
}

impl Stage {
    /// Get a constant name describing this stage of the pipeline.
    #[inline]
    pub const fn name(&self) -> &'static str {
        match self {
            Self::Alignment(_) => "alignment",
            Self::Stacking(_) => "stacking",
            Self::Sharpen(_) => "sharpen",
        }
    }
}

/// Represents a pipeline of operations on a group of entries.
pub struct Pipeline {
    pub stages: Vec<Stage>,
}

impl Pipeline {
    /// Run this pipeline.
    pub fn process<'scope>(
        &self,
        mut input: Entries<'scope, OwnedEntryIter<'scope>>,
    ) -> Result<Entries<'scope, OwnedEntryIter<'scope>>> {
        let span = tracing::info_span!("pipeline");
        let _enter = span.enter();

        // A pipeline stage needs to output an `Entry`. We let each stage
        // decide if holding the image in memory, or writing it to disk
        // is more efficient.
        for stage in &self.stages {
            tracing::info!(stage = %stage.name());
            input = match stage {
                Stage::Alignment(o) => alignment::process(input, o)?,
                Stage::Stacking(o) => stacking::process(input, o)?,
                Stage::Sharpen(o) => sharpen::process(input, o)?,
            }
        }
        Ok(input)
    }
}
