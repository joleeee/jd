use std::{env, fs::File, io::Read, process::exit};

use crate::color::Color;

const HEADER: [u8; 8] = [102, 97, 114, 98, 102, 101, 108, 100];

pub fn dither<R: std::io::Read, W: std::io::Write>(stdin: &mut R, stdout: &mut W) {
    let mut buffer = [0; 8];

    stdin.read_exact(&mut buffer).unwrap();

    if buffer != HEADER {
        eprintln!("Wrong header!");
        exit(1);
    }

    let mut bwidth = [0; 4];
    let mut bheight = [0; 4];
    stdin.read_exact(&mut bwidth).unwrap();
    stdin.read_exact(&mut bheight).unwrap();
    let width = u32::from_be_bytes(bwidth) as usize;
    let height = u32::from_be_bytes(bheight) as usize;

    let argument = env::args().nth(1);
    let pal = match argument {
        Some(filename) => read_pal(&filename),
        None => {
            const MAX: i32 = u16::MAX as i32;
            vec![
                Color { r: 0, g: 0, b: 0 },
                Color {
                    r: MAX,
                    g: MAX,
                    b: MAX,
                },
                Color { r: MAX, g: 0, b: 0 },
                Color { r: 0, g: MAX, b: 0 },
                Color { r: 0, g: 0, b: MAX },
            ]
        }
    };

    let mut out = vec![];
    let mut raw = Vec::new();
    stdin.read_to_end(&mut raw).unwrap();

    let mut data: Vec<_> = raw
        .chunks_exact(8)
        .map(|pixel| {
            let r = u16::from_be_bytes(pixel[0..2].try_into().unwrap()) as i32;
            let g = u16::from_be_bytes(pixel[2..4].try_into().unwrap()) as i32;
            let b = u16::from_be_bytes(pixel[4..6].try_into().unwrap()) as i32;
            Color { r, g, b }
        })
        .collect();

    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            let color = &data[i];
            let closest = closest_color(&pal, color);
            let diff = *color - *closest;

            if x < width - 1 {
                data[i + 1] += (diff * 7) / 16;
            }

            if y < height - 1 {
                if x < width - 1 {
                    data[i + width + 1] += diff / 16;
                }
                if x > 0 {
                    data[i + width - 1] += (diff * 3) / 16;
                }
                data[i + width] += (diff * 5) / 16;
            }

            out.push(closest);
        }
    }

    stdout.write_all(&HEADER).unwrap();
    let ne_width = (width as u32).to_be_bytes();
    let ne_height = (height as u32).to_be_bytes();
    stdout.write_all(&ne_width).unwrap();
    stdout.write_all(&ne_height).unwrap();
    for v in out {
        let r = (v.r as u16).to_be_bytes();
        let g = (v.g as u16).to_be_bytes();
        let b = (v.b as u16).to_be_bytes();
        let a = u16::MAX.to_be_bytes();
        stdout.write_all(&r).unwrap();
        stdout.write_all(&g).unwrap();
        stdout.write_all(&b).unwrap();
        stdout.write_all(&a).unwrap();
    }
}

fn closest_color<'a>(pal: &'a Vec<Color>, src: &Color) -> &'a Color {
    let mut best_diff = f64::MAX;
    let mut best_col = &pal[0];
    for col in pal {
        let diff = src.diff(col);

        if diff < best_diff {
            best_diff = diff;
            best_col = col;
        }
    }

    best_col
}

fn read_pal(filename: &str) -> Vec<Color> {
    let mut buf = String::new();
    let mut f = File::open(filename).unwrap();

    f.read_to_string(&mut buf).unwrap();

    const MUL: i32 = (u8::MAX as i32) + 1;

    buf.lines()
        .map(|line| {
            let mut rcol = hex::decode(line)
                .unwrap()
                .into_iter()
                .map(Into::<i32>::into);

            Color {
                r: rcol.next().unwrap(),
                g: rcol.next().unwrap(),
                b: rcol.next().unwrap(),
            } * MUL
        })
        .collect()
}
