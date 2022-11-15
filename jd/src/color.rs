use std::{
    fs::File,
    io::Read,
    ops::{AddAssign, Div, Mul, Sub},
};

/// The step in color values
const COL_STEP: i32 = (u8::MAX as i32) + 1;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: i32,
    pub g: i32,
    pub b: i32,
}

impl Div<i32> for Color {
    type Output = Self;
    fn div(self, rhs: i32) -> Self::Output {
        Color {
            r: self.r / rhs,
            g: self.g / rhs,
            b: self.b / rhs,
        }
    }
}

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl Mul<i32> for Color {
    type Output = Self;
    fn mul(self, rhs: i32) -> Self::Output {
        Color {
            r: self.r * rhs,
            g: self.g * rhs,
            b: self.b * rhs,
        }
    }
}

impl Sub for Color {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self::Output {
        Color {
            r: self.r - rhs.r,
            g: self.g - rhs.g,
            b: self.b - rhs.b,
        }
    }
}

impl Color {
    /// A higher score indicates a larger perceived difference in color
    pub fn diff(&self, other: &Self) -> f64 {
        let rmean = (self.r + other.r) as f64;
        let diff = *self - *other;
        let dr = diff.r.abs() as f64;
        let dg = diff.g.abs() as f64;
        let db = diff.b.abs() as f64;

        const MAX16: f64 = u16::MAX as f64;

        ((512.0 + rmean / MAX16) * dr * dr
            + 1024.0 * dg * dg
            + (512.0 + (MAX16 - 1.0 - rmean) / MAX16) * db * db)
            .sqrt()
    }
}

pub struct Palette {
    pub colors: Vec<Color>,
}

impl Default for Palette {
    fn default() -> Self {
        const MAX: i32 = u16::MAX as i32;
        Self {
            colors: vec![
                Color { r: 0, g: 0, b: 0 },
                Color {
                    r: MAX,
                    g: MAX,
                    b: MAX,
                },
                Color { r: MAX, g: 0, b: 0 },
                Color { r: 0, g: MAX, b: 0 },
                Color { r: 0, g: 0, b: MAX },
            ],
        }
    }
}

impl Palette {
    pub fn closest_color<'a>(self: &'a Palette, src: &Color) -> &'a Color {
        let mut best_diff = f64::MAX;
        let mut best_col = &self.colors[0];
        for col in &self.colors {
            let diff = src.diff(col);

            if diff < best_diff {
                best_diff = diff;
                best_col = col;
            }
        }

        best_col
    }

    pub fn from_file(filename: &str) -> Self {
        let mut buf = String::new();
        let mut f = File::open(filename).unwrap();

        f.read_to_string(&mut buf).unwrap();

        let colors = buf
            .lines()
            .map(|line| {
                let mut rcol = hex::decode(line)
                    .unwrap()
                    .into_iter()
                    .map(Into::<i32>::into);

                Color {
                    r: rcol.next().unwrap(),
                    g: rcol.next().unwrap(),
                    b: rcol.next().unwrap(),
                } * COL_STEP
            })
            .collect();

        Self { colors }
    }
}
