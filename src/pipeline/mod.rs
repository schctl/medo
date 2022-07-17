//! Defines an entry group's processing pipeline.

use crate::entry::{Entries, OwnedEntryIter};
use crate::Result;

pub mod alignment;
pub mod stacking;

/// A stage in the processing pipeline of a group of entries.
pub enum Stage {
    Alignment(alignment::Opts),
    Stacking(stacking::Opts),
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
        for stage in &self.stages {
            input = match stage {
                Stage::Alignment(o) => alignment::process(input, o)?,
                Stage::Stacking(o) => stacking::process(input, o)?,
            }
        }
        Ok(input)
    }
}
