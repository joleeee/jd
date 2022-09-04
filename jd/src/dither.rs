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
            let i = y * width + x;
            let color = &data[i as usize];
            let closest = *pal.closest_color(color);
            let diff = *color - closest;

            if x < width - 1 {
                data[i as usize + 1] += (diff * 7) / 16;
            }

            if y < height - 1 {
                let below = (i + width) as usize;
                if x < width - 1 {
                    data[below + 1] += diff / 16;
                }
                if x > 0 {
                    data[below - 1] += (diff * 3) / 16;
                }
                data[below] += (diff * 5) / 16;
            }

            out.push(closest);
        }
    }

    Image { data: out, ..img }
}
