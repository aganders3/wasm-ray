use crate::ray::Color;
use crate::vec3::{Vec3, Point};


pub enum Material {
    NormalView,
    VantaBlack,
    Metal{color: Color, fuzz: f32},
    Lambertian{color: Color, fuzz: f32},
}

impl Material {
    pub fn scatter(&self, incoming: Vec3, normal: Vec3) -> Option<Vec3> {
        match self {
            Self::NormalView => None,
            Self::VantaBlack => None,
            Self::Metal{color: _, fuzz} => {
                let direction = incoming - 2.0 * incoming.dot(&normal) * normal;
                let d = Self::reflect(direction, *fuzz);
                Some(d)
            },
            Self::Lambertian{color: _, fuzz} => {
                let d = Self::reflect(normal, *fuzz);
                Some(d)
            },
        }
    }

    pub fn color(&self, incoming: Vec3, normal: Vec3) -> Color {
        match self {
            Self::NormalView => Color::from_normal(&normal),
            Self::VantaBlack => Color{r: 0, g: 0, b: 0, a: 255},
            Self::Metal{color, fuzz: _} | Self::Lambertian{color, fuzz: _} => {
                *color
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