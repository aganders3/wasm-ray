use rand::prelude::*;

use crate::vec3::{Vec3, Point};

#[derive(Clone, Copy, Debug)]
pub struct Color{
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl Color {
    pub fn from_normal(normal: &Vec3) -> Color{
        Color {
            r: 0.5 + 0.5 * normal.x,
            g: 0.5 + 0.5 * normal.y,
            b: 0.5 + 0.5 * normal.z,
        }
    }

    pub fn random() -> Color{
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen::<f32>(),
            g: rng.gen::<f32>(),
            b: rng.gen::<f32>(),
        }
    }
}

impl std::ops::Mul<Color> for f32 {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        let factor: f32;
        if self > 0.0 && self < 1.0 {
            factor = self;
        } else {
            factor = 1.0;
        }

        Color{
            r: factor * _rhs.r,
            g: factor * _rhs.g,
            b: factor * _rhs.b,
        }
    }
}

impl std::ops::Mul<Color> for Color {
    type Output = Color;

    fn mul(self, _rhs: Color) -> Color {
        Color{
            r: self.r * _rhs.r,
            g: self.g * _rhs.g,
            b: self.b * _rhs.b,
        }
    }
}

impl std::cmp::PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        let e = 1e-8;
        (self.r - other.r).abs() < e && (self.g - other.g).abs() < e && (self.b - other.b).abs() < e
    }
}

pub fn blend(colors: Vec<Color>) -> Color {
    let mut r = 0.0;
    let mut g = 0.0;
    let mut b = 0.0;
    let len = colors.len() as f32;
    for color in colors.iter() {
        r += color.r;
        g += color.g;
        b += color.b;
    }

    Color{
        r: r / len,
        g: g / len,
        b: b / len,
    }
}

#[derive(Clone, Copy, Debug)]
pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
    pub depth: u8,
    pub color: Color,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }

    pub fn background_color(&self) -> Color {
        let t = 0.5 * (self.direction.unit().y + 1.0);
        Color{
            r: (1.0 - t) * 1.0 + t * 0.5,
            g: (1.0 - t) * 1.0 + t * 0.7,
            b: 1.0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blend_colors() {
        let colors = vec![
            Color{r: 0.0, g: 0.0, b: 0.0},
            Color{r: 0.5, g: 0.5, b: 0.5},
            Color{r: 1.0, g: 1.0, b: 1.0},
        ];

        let c = blend(colors);

        assert!(c.r == 0.5 && c.g == 0.5 && c.b == 0.5);

    }
}
