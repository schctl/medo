use std::borrow::Cow;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::path::PathBuf;

use opencv::core::Mat;
use rayon::iter::{IntoParallelIterator, ParallelBridge, ParallelIterator};

use medo_stacker::contour;
use medo_stacker::homography;
use medo_stacker::stacker::average::Stacker;
use medo_stacker::star;

use crate::util;
use crate::{Entries, Entry, Error, PipelineInput, PipelineOutput, PipelineStage, Result};

/// A group of entries and associated processing options.
pub struct Group {
    /// The name of this group.
    pub name: String,
    /// The current group of entries.
    ///
    /// The reference entry should never be mutated.
    pub entries: Entries,
    pub pipeline: Vec<PipelineStage>,
}

impl Group {
    /// Process this group's pipeline.
    pub fn process(&self) -> Result<Mat> {
        let mut p_input = PipelineInput {
            iter: Box::new(self.entries.entries.iter().map(|e| Cow::Borrowed(e))),
        };
        tracing::info!("Beginning pipeline");
        for stage in &self.pipeline {
            let out = match stage {
                PipelineStage::Alignment => self.align(p_input)?,
                PipelineStage::Stacking => self.stack(p_input)?,
            };
            match out {
                PipelineOutput::Ending(d) => {
                    tracing::info!("Pipeline done");
                    return Ok(d);
                }
                PipelineOutput::Piping(n) => {
                    p_input = n;
                }
            }
        }
        tracing::info!("Pipeline trailed");
        Err(Error::PipelineStageFailed)
    }

    /// The stacking stage of the pipeline.
    fn stack<'a>(&'a self, input: PipelineInput<'a>) -> Result<PipelineOutput> {
        let iter = [Cow::Borrowed(&self.entries.reference)]
            .into_iter()
            .chain(input.iter)
            .map(|e| util::read_image(e.path()).unwrap());

        tracing::info!("Stacking {}", self.name);
        let stacker = Stacker::new(iter);
        let data = stacker
            .filter_map(medo_stacker::Result::ok)
            .last()
            .ok_or(Error::PipelineStageFailed)?;
        tracing::info!("Done stacking {}", self.name);
        Ok(PipelineOutput::Ending(data))
    }

    /// The alignment stage of the pipeline.
    fn align<'a>(&self, input: PipelineInput<'a>) -> Result<PipelineOutput> {
        use opencv::core::{MatTraitConst, MatTraitConstManual, Scalar};
        use opencv::imgproc;

        // Hash entries
        let mut hasher = DefaultHasher::new();
        self.entries.hash(&mut hasher);
        let hash = hasher.finish();

        let construct_out_path = |e: &Entry| -> PathBuf {
            // Get path
            let mut out_path = util::temp_dir();
            out_path.push(format!("{:x}", hash));
            out_path.push(e.file_name());
            out_path
        };

        // Create alignment calculator
        tracing::debug!("Preparing reference...");
        let first = util::read_image(self.entries.reference.path())?;
        let first_mask = star::find_contours(&first, Default::default())?.collect();
        let first_mask = contour::create_mask(first.size()?, first.typ(), &first_mask)?;
        let calculator = homography::Calculator::new(&first_mask)?;
        tracing::debug!("Done preparing reference");

        // Align images
        let images = input
            .iter
            .par_bridge()
            .into_par_iter()
            .map(move |e| {
                // Start
                let path = e.path();
                let path_str = path.to_string_lossy();
                let start = std::time::Instant::now();
                tracing::info!("Aligning {}...", path_str,);
                // Read image
                let image = util::read_image(&path)?;
                // Create mask
                let mask = star::find_contours(&image, Default::default())?.collect();
                let mask = contour::create_mask(image.size()?, image.typ(), &mask)?;
                // Align
                let warp = calculator.calculate(&mask, Default::default())?;
                let mut dst = Mat::default();
                imgproc::warp_perspective(
                    &image,
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
                    "[{}s] Aligned {} -> {}",
                    start.elapsed().as_secs(),
                    path_str,
                    out_path.to_string_lossy()
                );
                Ok(Cow::Owned(Entry::new_owned(out_path)?))
            })
            .collect::<Result<Vec<_>>>()?
            .into_iter();

        Ok(PipelineOutput::Piping(PipelineInput {
            iter: Box::new(images),
        }))
    }
}
