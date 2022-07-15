//! Method of stacking by averaging.

use opencv::core::Mat;

pub struct Stacker<T: Iterator<Item = Mat>> {
    out: Mat,
    iter: T,
    prog: usize,
}

impl<T: Iterator<Item = Mat>> Stacker<T> {
    pub fn new<F: IntoIterator<Item = T::Item, IntoIter = T>>(iter: F) -> Self {
        let mut iter = iter.into_iter();
        Self {
            out: iter.next().unwrap(),
            iter,
            prog: 0,
        }
    }
}

impl<T: Iterator<Item = Mat>> Iterator for Stacker<T> {
    type Item = Mat;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|next| {
            // Calculate weight
            let alpha = 1.0 / (self.prog + 2) as f64;
            let beta = 1.0 - alpha;
            // Add
            let mut new = Mat::default();
            let _ = opencv::core::add_weighted(&next, alpha, &self.out, beta, 0.0, &mut new, -1);
            // Update progress
            self.out = new;
            self.prog += 1;

            self.out.clone()
        })
    }
}

impl<T: Iterator<Item = Mat>> super::Stacker for Stacker<T> {}

#[cfg(test)]
mod test {
    use super::*;

    use opencv::core::prelude::MatExprTraitConst;
    use opencv::core::prelude::MatTraitConst;

    /// Check if the result of stacking identical matrices is itself.
    #[test]
    fn identical_mats_stack_to_itself() {
        const SIZE: i32 = 5;

        // Create vec of identity matrices
        let imat = Mat::eye(SIZE, SIZE, opencv::core::CV_32F)
            .unwrap()
            .to_mat()
            .unwrap();
        let mats = (0..SIZE).map(|_| imat.clone()).collect::<Vec<_>>();

        // Stack
        let stacker = Stacker::new(mats);
        let last = stacker.last().unwrap();

        // Assert that average is an identity matrix
        for i in 0..SIZE {
            assert_eq!(*last.at_nd::<f32>(&[i, i]).unwrap(), 1.0);
        }
    }

    /// Try stacking four matrices with one's in each corner.
    #[test]
    fn four_cornered_mats() {
        // Initialize
        let mats = [
            Mat::from_slice_2d(&[[1.0_f32, 0.0_f32], [0.0_f32, 0.0_f32]]).unwrap(),
            Mat::from_slice_2d(&[[0.0_f32, 1.0_f32], [0.0_f32, 0.0_f32]]).unwrap(),
            Mat::from_slice_2d(&[[0.0_f32, 0.0_f32], [1.0_f32, 0.0_f32]]).unwrap(),
            Mat::from_slice_2d(&[[0.0_f32, 0.0_f32], [0.0_f32, 1.0_f32]]).unwrap(),
        ];

        // Stack
        let stacker = Stacker::new(mats);
        let last = stacker.last().unwrap();

        // Check result
        for i in 0..2 {
            for j in 0..2 {
                assert_eq!(*last.at_nd::<f32>(&[i, j]).unwrap(), 0.25);
            }
        }
    }
}
