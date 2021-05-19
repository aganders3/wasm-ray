use crate::ray::Color;
use crate::vec3::{Vec3, Point};


pub enum Material {
    NormalView,
    VantaBlack,
    Metal{color: Color, albedo: f32, fuzz: f32},
    Lambertian{color: Color, albedo: f32},
}

impl Material {
    pub fn scatter(&self, incoming: Vec3, normal: Vec3) -> (Option<Vec3>, Color) {
        match self {
            Self::NormalView => (None, Color::from_normal(&normal)),
            Self::VantaBlack => (None, Color{r: 0, g: 0, b: 0, a: 255}),
            Self::Metal{color, albedo, fuzz} => {
                let direction = incoming - 2.0 * incoming.dot(&normal) * normal;
                let d = Self::reflect(direction, *fuzz);
                (Some(d), *color)
            },
            Self::Lambertian{color, albedo} => {
                let d = Self::reflect(normal, 1.0);
                (Some(d), *color)
            },
        }
    }

    fn reflect(direction: Vec3, spread: f32) -> Vec3 {
        if spread > 0.0 {
            let scatter_direction = Vec3::random_unit();
            direction + spread * scatter_direction
        } else {
            direction
        }
    }
}