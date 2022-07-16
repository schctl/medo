use std::borrow::Cow;
use std::io;
use std::path::{Path, PathBuf};

use rayon::prelude::*;

use opencv::core::{Mat, MatTraitConst, MatTraitConstManual, Scalar, Vector};
use opencv::imgcodecs;
use opencv::imgproc;

use medo_stacker::contour;
use medo_stacker::homography;
use medo_stacker::stacker::average::Stacker;
use medo_stacker::star;

/// Convert a path to string, mapping to an error.
fn path_to_str(path: &Path) -> Cow<'_, str> {
    path.to_string_lossy()
}

/// Get a file's name without extension from a path.
fn path_to_file_name(path: &Path) -> io::Result<&str> {
    // o_o
    let mut name = path
        .file_name()
        .ok_or(io::Error::from(io::ErrorKind::NotFound))?
        .to_str()
        .ok_or(io::Error::from(io::ErrorKind::NotFound))?;
    if let Some(ext) = path.extension() {
        name = name
            .strip_suffix(
                ext.to_str()
                    .ok_or(io::Error::from(io::ErrorKind::NotFound))?,
            )
            .ok_or(io::Error::from(io::ErrorKind::NotFound))?;
    }
    Ok(name)
}

/// Read a BGR image from the specified path.
fn read_image<P: AsRef<Path>>(path: P) -> anyhow::Result<Mat> {
    Ok(imgcodecs::imread(
        path_to_str(path.as_ref()).as_ref(),
        imgcodecs::IMREAD_COLOR,
    )?)
}

/// Write an image to the specified path, and create directories if needed.
fn write_image<P: AsRef<Path>>(path: P, image: &Mat) -> anyhow::Result<()> {
    let path = path.as_ref();
    // Create parent directory if it doesn't exist
    if let Some(p) = path.parent() {
        if !p.exists() {
            std::fs::create_dir_all(p)?;
        }
    }
    // Write
    imgcodecs::imwrite(path_to_str(path).as_ref(), &image, &Vector::new())?;
    Ok(())
}

fn main() {
    // Initialization
    env_logger::builder()
        .filter(None, log::LevelFilter::Info)
        .init();
    rayon::ThreadPoolBuilder::new()
        .num_threads(4)
        .build_global()
        .unwrap();

    // Parse opts
    let mut args = std::env::args();
    let src_dir = PathBuf::from(&args.nth(1).unwrap());
    let out_path = PathBuf::from(&args.next().unwrap());
    let entries = std::fs::read_dir(&src_dir).unwrap();

    // Identifier for this run
    let run_id = path_to_file_name(&src_dir).unwrap();

    // Filter files
    let mut iter = entries.filter_map(|e| e.ok().filter(|v| v.path().is_file()));

    // Create alignment calculator
    let first = read_image(iter.next().unwrap().path()).unwrap();
    let first_mask = star::find_contours(&first, Default::default())
        .unwrap()
        .collect();
    let first_mask = contour::create_mask(first.size().unwrap(), first.typ(), &first_mask).unwrap();
    let calculator = homography::Calculator::new(&first_mask).unwrap();

    // Align images
    let images = iter
        .par_bridge()
        .map(move |p| {
            // Read image
            let path = p.path();
            let path_str = path_to_str(&path);
            let outp_str = format!(
                "/tmp/medo/{}/{}tif",
                run_id,
                path_to_file_name(&path).unwrap()
            );
            log::info!("Aligning {}... [out: {}]", path_str, outp_str);
            let image = read_image(&path).unwrap();

            // Create mask
            let mask = star::find_contours(&image, Default::default())
                .unwrap()
                .collect();
            let mask = contour::create_mask(image.size().unwrap(), image.typ(), &mask).unwrap();

            // Align
            let warp = calculator.calculate(&mask, Default::default()).unwrap();
            let mut dst = Mat::default();
            imgproc::warp_perspective(
                &image,
                &mut dst,
                &warp,
                image.size().unwrap(),
                imgproc::INTER_LINEAR,
                opencv::core::BORDER_CONSTANT,
                Scalar::default(),
            )
            .unwrap();

            // Write
            write_image(&outp_str, &image).unwrap();
            log::info!("Aligned {}", path_str);
            path.to_owned()
        })
        // Collect to take advantage of parallel alignment
        .collect::<Vec<PathBuf>>()
        .into_iter()
        .map(|p| read_image(&p).unwrap());

    // Stack
    let images = images.chain([first]);
    let stacker = Stacker::new(images);
    log::info!("Stacking...");
    let res = stacker.last().unwrap();

    // Write result
    log::info!("Writing output image...");
    write_image(&out_path, &res).unwrap();
    log::info!("Done!");
}
