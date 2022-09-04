use std::{fs::File, io::Cursor, process::Command};

use criterion::{black_box, criterion_group, criterion_main, BatchSize, Criterion};
use jd::{self, color::Palette, Image};

fn ipath(file: &str) -> String {
    format!("{}/benches/img/{}", env!("CARGO_MANIFEST_DIR"), file)
}

fn read_src(path: &str) -> Image {
    let src_jpg = File::open(path).unwrap();
    let src_ff = Command::new("2ff").stdin(src_jpg).output().unwrap().stdout;
    jd::ff::decode(&mut Cursor::new(src_ff)).unwrap()
}

pub fn bw_benchmark(c: &mut Criterion) {
    for (name, jpg, pal, _out) in [
        ("bw", "bw.jpg", "bw.pal", "bw_out.ff"),
        ("bw_small", "bw_small.jpg", "bw.pal", "bw_small_out.ff"),
        (
            "computer",
            "computer.jpg",
            "computer.pal",
            "computer_out.ff",
        ),
    ] {
        let img = black_box(read_src(&ipath(jpg)));
        let pal = black_box(Palette::from_file(&ipath(pal)));

        // just to make sure it's actually doing anything
        //let dithered = jd::dither(img.clone(), &pal);
        //let mut ff_output = File::create(&ipath(_out)).unwrap();
        //jd::ff::encode(&dithered, &mut ff_output);

        c.bench_function(name, |b| {
            b.iter_batched(
                || img.clone(),
                |img| jd::dither(img, &pal),
                BatchSize::NumIterations(5),
            )
        });
    }
}

criterion_group!(benches, bw_benchmark);
criterion_main!(benches);
