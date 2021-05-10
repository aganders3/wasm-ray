use crate::vec3::{Vec3, Point};

#[derive(Clone, Copy, Debug)]
pub struct Color{
    pub r: u8,
    pub g: u8,
    pub b:u8,
    pub a: u8
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

