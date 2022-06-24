use std::{env::args, io};

use jd::color::Palette;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());
    let palette = Palette::from_args(args());
    jd::dither_io(&mut stdin, &mut stdout, &palette);
}
