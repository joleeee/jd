use crate::{color::Palette, Image};

pub fn dither(img: Image, pal: &Palette) -> Image {
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
            let closest = *pal.closest_color(color);
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
