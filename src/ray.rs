use crate::vec3::{Vec3, Point};

#[derive(Clone, Copy, Debug)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}

impl Color {
    pub fn from_normal(normal: Vec3) -> Color{
        Color {
            r: (128. + 128.*normal.x) as u8,
            g: (128. + 128.*normal.y) as u8,
            b: (128. + 128.*normal.z) as u8,
            a: 255,
        }
    }
}

pub fn blend(colors: Vec<Color>) -> Color {
    let mut r: u32 = 0;
    let mut g: u32 = 0;
    let mut b: u32 = 0;
    let mut a: u32 = 0;
    let len = colors.len() as u32;
    for color in colors.iter() {
        r += color.r as u32;
        g += color.g as u32;
        b += color.b as u32;
        a += color.a as u32;
    }

    Color{
        r: (r / len) as u8,
        g: (g / len) as u8,
        b: (b / len) as u8,
        a: (a / len) as u8,
    }
}

pub struct Ray {
    pub origin: Point,
    pub direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Point {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let t = 0.5 * (self.direction.unit().y + 1.0);
        Color{
            r: ((1.0 - t) * 255. + t * 128.) as u8,
            g: ((1.0 - t) * 255. + t * 178.) as u8,
            b: 255,
            a: 255,
        }
    }
}

