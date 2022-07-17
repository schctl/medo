//! Implementation of the alignment stage.

use std::borrow::Cow;
use std::path::PathBuf;

use opencv::core::{Mat, MatTraitConst, MatTraitConstManual, Scalar};
use opencv::imgproc;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use medo_stacker::contour;
use medo_stacker::homography;
use medo_stacker::star;

use crate::entry::{Entries, Entry, OwnedEntryIter};
use crate::util;
use crate::Result;

#[derive(Debug, Clone, Default)]
pub struct Opts {}

pub fn process<'scope>(
    input: Entries<'scope, OwnedEntryIter<'scope>>,
    _opts: &Opts,
) -> Result<Entries<'scope, OwnedEntryIter<'scope>>> {
    let span = tracing::info_span!("stage_alignment");
    let _enter = span.enter();

    // // Hash entries
    // let mut hasher = DefaultHasher::new();
    // self.entries.hash(&mut hasher);
    // let hash = hasher.finish();

    let construct_out_path = |e: &Entry| -> PathBuf {
        // Get path
        let mut out_path = util::temp_dir();
        out_path.push(e.name().as_ref());
        out_path
    };

    // Create alignment calculator
    let first = input.reference.read_image()?;
    let first_mask = star::find_contours(&first, Default::default())?.collect();
    let first_mask = contour::create_mask(first.size()?, first.typ(), &first_mask)?;
    let calculator = homography::Calculator::new(&first_mask)?;

    // Align images
    let images = input
        .entries
        .par_bridge()
        .into_par_iter()
        .map(move |e| {
            let span = tracing::debug_span!("ps_alignment_unit");
            let _enter = span.enter();

            // Start
            let name = e.name();
            let start = std::time::Instant::now();
            tracing::info!(%name, "aligning...");
            // Read image
            let image = e.read_image()?;
            // Create mask
            let mask = star::find_contours(&image, Default::default())?.collect();
            let mask = contour::create_mask(image.size()?, image.typ(), &mask)?;
            // Align
            let warp = calculator.calculate(&mask, Default::default())?;
            let mut dst = Mat::default();
            imgproc::warp_perspective(
                image.as_ref(),
                &mut dst,
                &warp,
                image.size().unwrap(),
                imgproc::INTER_LINEAR,
                opencv::core::BORDER_CONSTANT,
                Scalar::default(),
            )?;
            // Write
            let out_path = construct_out_path(&e);
            util::write_image(&out_path, &image)?;
            // Done
            tracing::info!(
                %name,
                output = out_path.to_string_lossy().as_ref(),
                time = %format!("{}s", start.elapsed().as_secs()),
                "done aligning",
            );
            Ok(Cow::Owned(Entry::new_path_owned(out_path)?))
        })
        .collect::<Result<Vec<_>>>()?
        .into_iter();

    Ok(Entries {
        reference: input.reference,
        entries: Box::new(images),
    })
}
