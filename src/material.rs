use rand::prelude::*;

use crate::ray::Color;
use crate::vec3::Vec3;

pub enum Material {
    NormalView,
    VantaBlack,
    Metal{color: Color, fuzz: f32},
    Lambertian{color: Color},
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
                // TODO: weird "glowing rim" effect if this is included
                // if !d.near_zero() && d.dot(&normal) > 0.0 { Some(d) } else { None }
                // TODO: does not seem helpful
                // if !d.near_zero() { Some(d) } else { Some(incoming) }
                Some(d)
            },
            Self::Lambertian{color: _} => {
                let d = Self::reflect(normal, 1.0);
                // if ! d.near_zero() && d.dot(&normal) > 0.0 { Some(d) } else { None }
                if !d.near_zero() { Some(d) } else { Some(normal) }
            },
            Self::Dielectric{color: _, eta} => {
                let d = Self::reflect_or_refract(incoming, normal, front_face, *eta);
                if !d.near_zero() { Some(d) } else { None }
            },
        }
    }

    pub fn color(&self, normal: Vec3) -> Color {
        // TODO: check if front face for coloring as well?
        match self {
            Self::NormalView => Color::from_normal(&normal),
            Self::VantaBlack => Color{r: 0.0, g: 0.0, b: 0.0},
            Self::Metal{color, fuzz: _} | Self::Lambertian{color} | Self::Dielectric{color, eta: _} => {
                *color
            },
        }
    }

    fn reflect(direction: Vec3, spread: f32) -> Vec3 {
        if spread > 0.0 {
            let scatter_direction = Vec3::random_in_unit_sphere().unit();
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
        // assume transition is always:
        //   air-to-dielectric (front face)
        //   or dielectric-to-air (inside face)
        let refraction_ratio = if front_face { 1.0 / eta } else { eta };
        let incoming_unit = incoming.unit();
        let cos_theta = (-incoming_unit).dot(&normal).min(1.0);

        // always refract
        // Self::refract(incoming_unit, normal, cos_theta, refraction_ratio)

        // sometimes reflect, sometimes refract
        let sin_theta = (1.0 - cos_theta*cos_theta).sqrt();
        let randf = thread_rng().gen::<f32>();
        if refraction_ratio * sin_theta > 1.0 || Self::reflectance(cos_theta, refraction_ratio) > randf {
            let direction = incoming - 2.0 * incoming.dot(&normal) * normal;
            Self::reflect(direction, 0.0)
        } else {
            Self::refract(incoming_unit, normal, cos_theta, refraction_ratio)
        }
    }

    fn reflectance(cos_theta: f32, refraction_ratio: f32) -> f32 {
        // Schlick's approximation for reflectance
        let r0 = ((1.0 - refraction_ratio) / (1.0 + refraction_ratio)).powf(2.0);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powf(5.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scatter_normalview() {
        let incoming = Vec3{x: 0.0, y: 0.0, z: -1.0};
        let normal = Vec3{x: 0.0, y: 0.0, z: 1.0};

        let scatter = Material::NormalView.scatter(incoming, normal, true);
        assert!(scatter.is_none());

        let color = Material::NormalView.color(normal);
        assert_eq!(color, Color{r: 0.5, g: 0.5, b: 1.0});
    }

    #[test]
    fn scatter_vantablack() {
        let incoming = Vec3{x: 0.0, y: 0.0, z: -1.0};
        let normal = Vec3{x: 0.0, y: 0.0, z: 1.0};

        let scatter = Material::VantaBlack.scatter(incoming, normal, true);
        assert!(scatter.is_none());

        let color = Material::VantaBlack.color(normal);
        assert_eq!(color, Color{r: 0.0, g: 0.0, b: 0.0});
    }

    #[test]
    fn scatter_metal() {
        let metal = Material::Metal{color: Color{r: 1.0, g: 1.0, b: 1.0}, fuzz: 0.0};

        let incoming = Vec3{x: 0.0, y: 0.0, z: -1.0};
        let normal = Vec3{x: 0.0, y: 0.0, z: 1.0};

        let scatter = metal.scatter(incoming, normal, true);
        // TODO: test it's actually reflected properly
        assert!(scatter.is_some());

        let color = metal.color(normal);
        assert_eq!(color, Color{r: 1.0, g: 1.0, b: 1.0});
    }

    #[test]
    fn scatter_lambertian() {
        let lambertian = Material::Lambertian{color: Color{r: 1.0, g: 1.0, b: 1.0}};

        let incoming = Vec3{x: 0.0, y: 0.0, z: -1.0};
        let normal = Vec3{x: 0.0, y: 0.0, z: 1.0};

        let scatter = lambertian.scatter(incoming, normal, true);
        // TODO: test it's actually reflected properly
        assert!(scatter.is_some());

        let color = lambertian.color(normal);
        assert_eq!(color, Color{r: 1.0, g: 1.0, b: 1.0});
    }

    #[test]
    fn scatter_dielectric() {
        let dielectric = Material::Dielectric{color: Color{r: 1.0, g: 1.0, b: 1.0}, eta: 1.5};

        let incoming = Vec3{x: 0.0, y: 0.0, z: -1.0};
        let normal = Vec3{x: 0.0, y: 0.0, z: 1.0};

        let scatter = dielectric.scatter(incoming, normal, true);
        // TODO: test it's actually refracted properly
        assert!(scatter.is_some());

        let color = dielectric.color(normal);
        assert_eq!(color, Color{r: 1.0, g: 1.0, b: 1.0});
    }
}
