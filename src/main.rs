use std::io::{self, BufWriter};

use jd::dither;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = BufWriter::new(io::stdout());

    dither(&mut stdin, &mut stdout);
}
