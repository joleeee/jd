use std::io;

fn main() {
    let mut stdin = io::stdin();
    let mut stdout = io::BufWriter::new(io::stdout());
    jd::dither_io(&mut stdin, &mut stdout);
}
