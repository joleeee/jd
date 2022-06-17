use std::{env, fs::File, io::Read};

use crate::{color::Color, Image};

pub fn dither(img: Image) -> Image {
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

    let Image {
        width,
        height,
        mut data,
    } = img;

    let mut out = vec![];

    for y in 0..height {
        for x in 0..width {
            let i = (y * width + x) as usize;
            let color = &data[i];
            let closest = *closest_color(&pal, color);
            let diff = *color - closest;

            if x < width - 1 {
                data[i + 1] += (diff * 7) / 16;
            }

            if y < height - 1 {
                if x < width - 1 {
                    data[i + width as usize + 1] += diff / 16;
                }
                if x > 0 {
                    data[i + width as usize - 1] += (diff * 3) / 16;
                }
                data[i + width as usize] += (diff * 5) / 16;
            }

            out.push(closest);
        }
    }

    Image { data: out, ..img }
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
