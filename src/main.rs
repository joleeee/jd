use std::{env::args, io};

use jd::color::Palette;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());

    let palette = args()
        .nth(1)
        .map(|path| Palette::from_file(&path))
        .unwrap_or_default();

    jd::dither_io(&mut stdin, &mut stdout, &palette);
}
