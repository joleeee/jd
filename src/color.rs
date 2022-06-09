use std::ops::{AddAssign, Div, Mul, Sub};

#[derive(Clone, Copy)]
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
