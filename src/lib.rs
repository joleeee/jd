pub mod color;
pub mod ff;

mod dither;
pub use dither::dither;

use std::io;

pub struct Image {
    width: u32,
    height: u32,
    data: Vec<color::Color>,
}

pub fn dither_io<R: io::Read, W: io::Write>(stdin: &mut R, stdout: &mut W) {
    let input = ff::decode(stdin).unwrap();
    let output = dither(input);
    ff::encode(&output, stdout);
}
