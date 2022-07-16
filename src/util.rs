//! General purpose utilities.

use std::path::{Path, PathBuf};

use opencv::core::{Mat, Vector, VectorElement, VectorExtern};
use opencv::imgcodecs;

use crate::Result;

lazy_static::lazy_static! {
    static ref EMPTY_VEC_I32: OpaqueVector<i32> = OpaqueVector(Vector::new());
}

/// Opaque and sync wrapper for C++ vector.
struct OpaqueVector<T>(Vector<T>)
where
    Vector<T>: VectorExtern<T>,
    T: VectorElement;

unsafe impl<T> Sync for OpaqueVector<T>
where
    Vector<T>: VectorExtern<T>,
    T: VectorElement,
{
}

/// Get a temporary directory path to work with.
pub fn temp_dir() -> PathBuf {
    PathBuf::from("/tmp/medo")
}

/// Convenience method to write an image with default options.
pub fn write_image<P: AsRef<Path>>(path: P, image: &Mat) -> Result<()> {
    let path = path.as_ref();
    // Create parent directory if it doesn't exist
    if let Some(p) = path.parent() {
        if !p.exists() {
            std::fs::create_dir_all(p)?;
        }
    }
    imgcodecs::imwrite(path.to_string_lossy().as_ref(), &image, &EMPTY_VEC_I32.0)?;
    Ok(())
}

/// Convenience method to read a BGR image.
pub fn read_image<P: AsRef<Path>>(path: P) -> Result<Mat> {
    Ok(imgcodecs::imread(
        path.as_ref().to_string_lossy().as_ref(),
        imgcodecs::IMREAD_COLOR,
    )?)
}
