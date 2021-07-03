use crate::material::Material;
use crate::ray::{Ray, Color};
use crate::vec3::{Vec3, Point};

pub struct Hit {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub front_face: bool,
    pub scatter: Option<Ray>,
    pub color: Color,
}

pub trait Wobject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}

impl Wobject for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        let oc = ray.origin - self.center;
        let a = ray.direction.length_squared();
        let half_b = oc.dot(&ray.direction);
        let c = oc.length_squared() - (self.radius * self.radius);
        let discriminant = half_b.powi(2) - a * c;

        if discriminant > 0.0 {
            // TODO: this is a little ugly
            let t0 = (-half_b - discriminant.sqrt()) / a;
            let t1 = (-half_b + discriminant.sqrt()) / a;
            let t;
            if t0 > t_min && t0 < t_max {
                t = t0;
            } else if t1 > t_min && t1 < t_max {
                t = t1;
            } else {
                return None;
            }

            let p = ray.at(t);
            let outward_normal = (self.radius * (p - self.center)).unit();
            let front_face = ray.direction.dot(&outward_normal) < 0.0;
            let normal = if front_face { outward_normal } else { -outward_normal };

            let color = self.material.color(normal);

            // scattered ray
            let direction = self.material.scatter(ray.direction, normal, front_face);

            let scatter = direction.map(
                |d| Ray{origin: p, direction: d, depth: ray.depth + 1, color: ray.color * color}
            );

            return Some(Hit{p, normal, t, front_face, scatter, color});
        }
        None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn hit_sphere() {
        let sphere = Sphere {
            center: Point{x: 0.0, y: 0.0, z: -2.0},
            radius: 1.0,
            material: Material::NormalView,
        };

        let ray = Ray{
            origin: Point{x: 0.0, y: 0.0, z: 0.0},
            direction: Vec3{x: 0.0, y: 0.0, z: -1.0},
            depth: 0,
            color: Color{r: 1.0, g: 1.0, b: 1.0},
        };

        let hit = sphere.hit(&ray, 0.0, f32::INFINITY);

        assert!(hit.is_some());
    }

    #[test]
    fn miss_sphere() {
        let sphere = Sphere {
            center: Point{x: 0.0, y: 100.0, z: -2.0},
            radius: 1.0,
            material: Material::NormalView,
        };

        let ray = Ray{
            origin: Point{x: 0.0, y: 0.0, z: 0.0},
            direction: Vec3{x: 0.0, y: 0.0, z: -1.0},
            depth: 0,
            color: Color{r: 1.0, g: 1.0, b: 1.0},
        };

        let hit = sphere.hit(&ray, 0.0, f32::INFINITY);

        assert!(hit.is_none());
    }
}
