use opencv::core::Mat;

use super::Calculate;

pub struct Calculator {}

impl Calculate for Calculator {
    fn calculate(&mut self, src: Mat) -> Mat {
        panic!()
    }
}
