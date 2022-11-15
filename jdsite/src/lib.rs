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

#[wasm_bindgen]
pub fn jdither(buffer: &[u8]) -> Box<[u8]> {
    console_error_panic_hook::set_once();

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

        alert(&format!("Vec: {:?}", img_data.len()));

        let res = jd::dither(
            jd::Image {
                width: header.width,
                height: header.height,
                data: img_data,
            },
            &jd::color::Palette::default(),
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
