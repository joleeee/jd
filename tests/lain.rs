use std::{
    fs::File,
    io::{Cursor, Read},
    process::Command,
};

#[test]
fn mem() {
    fn ipath(file: &str) -> String {
        format!("{}/tests/img/{}", env!("CARGO_MANIFEST_DIR"), file)
    }

    // read the source
    let src_jpg = File::open(ipath("lain.jpg")).unwrap();
    let src_ff = Command::new("2ff").stdin(src_jpg).output().unwrap().stdout;

    // generate the dither and write to file
    let mut out = File::create(ipath("lain_out.ff")).unwrap();
    jd::dither_io(&mut Cursor::new(src_ff), &mut out);

    // read in the solution
    let ans_png = File::open(ipath("lain_ans.png")).unwrap();
    let ans_ff = Command::new("2ff").stdin(ans_png).output().unwrap().stdout;

    // read the generated dither
    let mut out_ff = Vec::new();
    File::open(ipath("lain_out.ff"))
        .unwrap()
        .read_to_end(&mut out_ff)
        .unwrap();

    assert_eq!(ans_ff, out_ff);
}
