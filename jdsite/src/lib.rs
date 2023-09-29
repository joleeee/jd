use std::io::Cursor;

use wasm_bindgen::prelude::*;

use jd::{self, color::Color};
use png;
use png_decoder;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Sup, {}!", name));
}

fn color_from_hex(chars: &[u8]) -> Color {
    fn nibble(c: u8) -> u8 {
        let c = c as char;
        match c {
            '0'..='9' => c as u8 - '0' as u8,
            'a'..='f' => (c as u8 - 'a' as u8) + 10,
            _ => 0,
        }
    }

    fn channel(c_l: u8, c_r: u8) -> u8 {
        nibble(c_l) * 16 + nibble(c_r)
    }

    let r = channel(chars[0], chars[1]);
    let g = channel(chars[2], chars[3]);
    let b = channel(chars[4], chars[5]);
    Color {
        r: (r as i32) * 256,
        g: (g as i32) * 256,
        b: (b as i32) * 256,
    }
}

#[wasm_bindgen]
pub fn jdither(buffer: &[u8], colors: String) -> Box<[u8]> {
    console_error_panic_hook::set_once();

    let palette: Vec<_> = colors
        .split(',')
        .map(|v| color_from_hex(v.as_bytes()))
        .collect();
    let palette = jd::color::Palette { colors: palette };

    let data = png_decoder::decode(buffer);

    if let Ok(data) = data {
        let (header, image_data) = data;

        let img_data: Vec<_> = image_data
            .chunks_exact(4)
            .map(|dat| Color {
                r: dat[0] as i32 * 256,
                g: dat[1] as i32 * 256,
                b: dat[2] as i32 * 256,
                // no alpha
            })
            .collect();

        let res = jd::dither(
            jd::Image {
                width: header.width,
                height: header.height,
                data: img_data,
            },
            &palette,
        );

        let img_data: Vec<_> = res
            .data
            .iter()
            .flat_map(|c| [c.r, c.g, c.b])
            .map(|v| (v / 256) as u8)
            .collect();

        let mut w = Cursor::new(Vec::new());
        let mut enc = png::Encoder::new(w.get_mut(), res.width, res.height);
        enc.set_color(png::ColorType::Rgb);
        enc.set_depth(png::BitDepth::Eight);

        let mut writer = enc.write_header().unwrap();
        writer.write_image_data(&img_data).unwrap();
        writer.finish().unwrap();

        w.into_inner().into()
    } else {
        let err = data.unwrap_err();
        alert(&format!("Failed: {:?}", err));

        panic!("");
    }
}
