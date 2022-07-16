use std::borrow::Cow;

use opencv::core::Mat;

use crate::Entry;

/// A stage in the processing pipeline of a group.
pub enum PipelineStage {
    Alignment,
    Stacking,
}

/// The input to a stage of a pipeline.
pub struct PipelineInput<'a> {
    pub data: Mat,
    pub iter: Box<dyn Iterator<Item = Cow<'a, Entry>> + Send + 'a>,
}

/// The output from a stage of a pipeline.
pub enum PipelineOutput<'a> {
    Ending(Mat),
    Piping(PipelineInput<'a>),
}
