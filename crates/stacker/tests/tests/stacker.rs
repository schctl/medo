use std::borrow::Cow;

use medo_core::cv::core::{MatTraitConst, Point3_};
use medo_core::entry::Entry;
use medo_stacker::stacker::Stacker;
use medo_stacker_tests::common;

#[test]
fn stack_average_binary() {
    let binary_1 =
        Entry::new_image("binary_1", common::read_image("binary_1.ppm").unwrap()).unwrap();
    let binary_2 =
        Entry::new_image("binary_2", common::read_image("binary_2.ppm").unwrap()).unwrap();
    let mut stacker = Stacker::average([Cow::Owned(binary_1), Cow::Owned(binary_2)]).unwrap();
    for i in stacker.by_ref() {
        i.unwrap();
    }
    let last = stacker.leak();
    let image = last.read_image().unwrap();

    for i in 0..image.rows() {
        for j in 0..image.cols() {
            assert_eq!(
                image.at_nd::<Point3_<u8>>(&[i, j]).unwrap(),
                &Point3_ {
                    x: 128,
                    y: 128,
                    z: 128
                }
            )
        }
    }
}
