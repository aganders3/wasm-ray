use rand::prelude::*;

use crate::ray::Color;
use crate::vec3::Vec3;


pub enum Material {
    NormalView,
    VantaBlack,
    Metal{color: Color, fuzz: f32},
    Lambertian{color: Color, fuzz: f32},
    Dielectric{color: Color, eta: f32},
}

impl Material {
    pub fn scatter(&self, incoming: Vec3, normal: Vec3, front_face: bool) -> Option<Vec3> {
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
            Self::Dielectric{color: _, eta} => {
                let d = Self::reflect_or_refract(incoming, normal, front_face, *eta);
                Some(d)
            },
        }
    }

    pub fn color(&self, incoming: Vec3, normal: Vec3) -> Color {
        match self {
            Self::NormalView => Color::from_normal(&normal),
            Self::VantaBlack => Color{r: 0.0, g: 0.0, b: 0.0},
            Self::Metal{color, fuzz: _} | Self::Lambertian{color, fuzz: _} | Self::Dielectric{color, eta: _} => {
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

    fn refract(incoming_unit: Vec3, normal: Vec3, cos_theta: f32, refraction_ratio: f32) -> Vec3 {
        let perp = refraction_ratio * (incoming_unit + cos_theta * normal);
        let parallel = -(1.0 - perp.length_squared()).abs().sqrt() * normal;
        perp + parallel
    }

    fn reflect_or_refract(incoming: Vec3, normal: Vec3, front_face: bool, eta: f32) -> Vec3 {
        let refraction_ratio = if front_face { 1.0 / eta } else { eta };

        let incoming_unit = incoming.unit();
        let cos_theta = (-incoming_unit).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();

        Self::refract(incoming_unit, normal, cos_theta, refraction_ratio)
        /*
        // sometimes reflect, sometimes refract
        let randf = thread_rng().gen::<f32>();
        if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > randf {
            let direction = incoming - 2.0 * incoming.dot(&normal) * normal;
            Self::reflect(direction, 0.0)
        } else {
            Self::refract(incoming_unit, normal, cos_theta, refraction_ratio)
        }
        */
    }

    fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
        // Schlick's approximation for reflectance
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
}
