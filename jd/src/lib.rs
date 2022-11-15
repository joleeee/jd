pub mod color;
pub mod ff;

mod dither;
use color::Palette;
pub use dither::dither;

use std::io;

#[derive(Clone)]
pub struct Image {
    pub width: u32,
    pub height: u32,
    pub data: Vec<color::Color>,
}

pub fn dither_io<R: io::Read, W: io::Write>(stdin: &mut R, stdout: &mut W, palette: &Palette) {
    let input = ff::decode(stdin).unwrap();
    let output = dither(input, palette);
    ff::encode(&output, stdout);
}
