use crate::color::Color;
use crate::Image;
use std::io;

#[derive(Debug)]
pub enum FFError {
    WrongHeader,
}

const HEADER: [u8; 8] = [102, 97, 114, 98, 102, 101, 108, 100];

pub fn decode<R: io::Read>(input: &mut R) -> Result<Image, FFError> {
    let mut buffer = [0; 8];

    input.read_exact(&mut buffer).unwrap();

    if buffer != HEADER {
        return Err(FFError::WrongHeader);
    }

    let mut bwidth = [0; 4];
    let mut bheight = [0; 4];
    input.read_exact(&mut bwidth).unwrap();
    input.read_exact(&mut bheight).unwrap();
    let width = u32::from_be_bytes(bwidth);
    let height = u32::from_be_bytes(bheight);

    let mut raw = Vec::new();
    input.read_to_end(&mut raw).unwrap();

    let data: Vec<_> = raw
        .chunks_exact(8)
        .map(|pixel| {
            let r = u16::from_be_bytes(pixel[0..2].try_into().unwrap()) as i32;
            let g = u16::from_be_bytes(pixel[2..4].try_into().unwrap()) as i32;
            let b = u16::from_be_bytes(pixel[4..6].try_into().unwrap()) as i32;
            Color { r, g, b }
        })
        .collect();

    Ok(Image {
        width,
        height,
        data,
    })
}

pub fn encode<W: io::Write>(img: &Image, output: &mut W) {
    output.write_all(&HEADER).unwrap();
    let ne_width = img.width.to_be_bytes();
    let ne_height = img.height.to_be_bytes();
    output.write_all(&ne_width).unwrap();
    output.write_all(&ne_height).unwrap();
    for v in &img.data {
        let r = (v.r as u16).to_be_bytes();
        let g = (v.g as u16).to_be_bytes();
        let b = (v.b as u16).to_be_bytes();
        let a = u16::MAX.to_be_bytes();
        output.write_all(&r).unwrap();
        output.write_all(&g).unwrap();
        output.write_all(&b).unwrap();
        output.write_all(&a).unwrap();
    }
}
