//! Implementation of the stacking stage.

use std::borrow::Cow;

use medo_core::entry::{Entries, OwnedEntryIter};
use medo_core::Result;
use medo_stacker::stacker::Stacker;

#[derive(Debug, Clone, Default)]
pub struct Opts {}

pub fn process<'scope>(
    input: Entries<'scope, OwnedEntryIter<'scope>>,
    _opts: &Opts,
) -> Result<Entries<'scope, OwnedEntryIter<'scope>>> {
    let span = tracing::info_span!("stage_stacking");
    let _enter = span.enter();

    let iter = [input.reference].into_iter().chain(input.entries);

    let mut stacker = Stacker::average(iter)?;
    for (n, r) in stacker.by_ref().enumerate() {
        // FIXME: identify image that failed to stack
        if let Err(e) = r {
            tracing::error!(index = n, error = %e, "failed to stack entry, discarding")
        }
    }

    Ok(Entries {
        reference: Cow::Owned(stacker.leak()),
        entries: Box::new(std::iter::empty()),
    })
}
