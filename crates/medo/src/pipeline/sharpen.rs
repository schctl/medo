//! Implementation of the sharpening stage.

use std::borrow::Cow;

use medo_core::cv;
use medo_core::cv::core::{Mat, Size};
use medo_core::cv::imgproc;
use medo_core::entry::{Entries, Entry, OwnedEntryIter};
use medo_core::Result;

#[derive(Debug, Clone, Default)]
pub struct Opts {}

/// Sharpen and return an owned entry.
fn sharpen(image: &Entry) -> Result<Entry> {
    let image_mat = image.read_image()?;
    let mut result_1 = Mat::default();
    imgproc::gaussian_blur(
        image_mat.as_ref(),
        &mut result_1,
        Size::new(0, 0),
        3.0,
        0.0,
        cv::core::BORDER_DEFAULT,
    )?;
    let mut result_2 = Mat::default();
    cv::core::add_weighted(
        image_mat.as_ref(),
        1.5,
        &result_1,
        -0.5,
        0.0,
        &mut result_2,
        -1,
    )?;

    Entry::new_image(image.name(), result_2)
}

pub fn process<'scope>(
    input: Entries<'scope, OwnedEntryIter<'scope>>,
    _opts: &Opts,
) -> Result<Entries<'scope, OwnedEntryIter<'scope>>> {
    Ok(Entries {
        reference: Cow::Owned(sharpen(input.reference.as_ref())?),
        entries: Box::new(input.entries.filter_map(|e| {
            let span = tracing::info_span!("stage_sharpen");
            let _enter = span.enter();

            let name = e.name();
            match sharpen(e.as_ref()) {
                Err(e) => {
                    tracing::error!(name = %name, error = %e, "failed to sharpen entry, discarding");
                    None
                }
                Ok(e) => {
                    Some(Cow::Owned(e))
                }
            }
        })),
    })
}
