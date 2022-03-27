use std::{process::Command, io::{Cursor, Read}, fs::File};

use jd::dither;

#[test]
fn mem() {
    fn ipath(file: &str) -> String {
        let mut s = String::new();
        s.push_str(env!("CARGO_MANIFEST_DIR"));
        s.push_str("/tests/img/");
        s.push_str(file);
        s
    }

    let src_jpg = File::open(ipath("lain.jpg")).unwrap();
    let src_ff = Command::new("2ff").stdin(src_jpg).output().unwrap().stdout;

    let mut out = File::create(ipath("lain_out.ff")).unwrap();

    dither(&mut Cursor::new(src_ff), &mut out);

    let ans_png = File::open(ipath("lain_ans.png")).unwrap();
    let ans_ff = Command::new("2ff").stdin(ans_png).output().unwrap().stdout;

    let mut out_ff = Vec::new();
    File::open(ipath("lain_out.ff")).unwrap().read_to_end(&mut out_ff).unwrap();
    
    assert_eq!(ans_ff, out_ff);
}
