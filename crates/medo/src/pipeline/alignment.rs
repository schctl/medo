//! Implementation of the alignment stage.

use std::borrow::Cow;
use std::path::PathBuf;

use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use medo_core::cv;
use medo_core::cv::core::{Mat, MatTraitConst, MatTraitConstManual, Scalar};
use medo_core::cv::imgproc;
use medo_core::entry::{Entries, Entry, OwnedEntryIter};
use medo_core::util;
use medo_core::Result;
use medo_stacker::homography;
use medo_stacker::star;

#[derive(Debug, Clone, Default)]
pub struct Opts {}

pub fn process<'scope>(
    input: Entries<'scope, OwnedEntryIter<'scope>>,
    _opts: &Opts,
) -> Result<Entries<'scope, OwnedEntryIter<'scope>>> {
    let span = tracing::info_span!("stage_alignment");
    let _enter = span.enter();

    let construct_out_path = |name: &str| -> PathBuf {
        // Get path
        let mut out_path = util::temp_dir();
        out_path.push(name);
        out_path
    };

    // Create alignment calculator
    let first = input.reference.read_image()?;
    let first_stars = star::find_contours(&first, Default::default())?;
    let first_mask = star::create_mask(first.size()?, first.typ(), first_stars)?;
    let calculator = homography::Calculator::new(&first_mask)?;

    // TODO: reliably find pre-aligned image
    // An alignment result is related to two imaages: the image itself
    // and the reference. Find some way to reliably hash both in case that
    // there is no absolute path to hash.

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
            let stars = star::find_contours(&image, Default::default())?;
            let mask = star::create_mask(image.size()?, image.typ(), stars)?;
            // Align
            let warp = calculator.calculate(&mask, Default::default())?;
            let mut dst = Mat::default();
            imgproc::warp_perspective(
                image.as_ref(),
                &mut dst,
                &warp,
                image.size().unwrap(),
                imgproc::INTER_LINEAR,
                cv::core::BORDER_CONSTANT,
                Scalar::default(),
            )?;
            // Write
            let out_path = construct_out_path(&name);
            util::write_image(&out_path, &dst)?;
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
