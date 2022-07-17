//! Implementation of the stacking stage.

use std::borrow::Cow;

use medo_stacker::stacker::average::Stacker;

use crate::entry::{Entries, Entry, OwnedEntryIter};
use crate::{Error, Result};

#[derive(Debug, Clone, Default)]
pub struct Opts {}

pub fn process<'scope>(
    input: Entries<'scope, OwnedEntryIter<'scope>>,
    _opts: &Opts,
) -> Result<Entries<'scope, OwnedEntryIter<'scope>>> {
    let span = tracing::info_span!("stage_stacking");
    let _enter = span.enter();

    let iter = [input.reference]
        .into_iter()
        .chain(input.entries)
        .map(|e| e.read_image().unwrap().into_owned());

    // FIXME: identify this run of the stacker
    tracing::info!("stacking...");
    let stacker = Stacker::new(iter);
    let data = stacker
        .filter_map(medo_stacker::Result::ok)
        .last()
        .ok_or(Error::PipelineStageFailed)?;
    tracing::info!("done stacking");

    Ok(Entries {
        reference: Cow::Owned(Entry::new_image("Stack Result", data)?),
        entries: Box::new(std::iter::empty()),
    })
}