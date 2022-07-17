//! Defines operations on groups of entries.

use crate::entry::OwnedEntries;
use crate::pipeline::Pipeline;
use crate::Result;

/// A group of entries and associated processing options.
pub struct Group {
    /// The name of this group.
    pub name: String,
    /// This group's pipeline.
    pub pipeline: Pipeline,
    /// The current group of entries.
    ///
    /// The reference entry should never be mutated.
    pub entries: OwnedEntries,
    /// The output of running this group's pipeline.
    pub pipeline_output: Option<OwnedEntries>,
}

impl Group {
    /// Process this group's pipeline.
    pub fn process(&mut self) -> Result<&OwnedEntries> {
        let span = tracing::info_span!("group", name = %self.name);
        let _enter = span.enter();

        self.pipeline_output = Some(
            self.pipeline
                .process(self.entries.to_borrow())?
                .into_owned(),
        );
        Ok(self.pipeline_output.as_ref().unwrap())
    }
}
