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
        let a = ray.direction.dot(&ray.direction);
        let b = 2.0 * oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;

        if discriminant >= 0.0 {
            let t = (-b - discriminant.sqrt()) / (2.0 * a);
            if t > t_min && t < t_max {
                let p = ray.at(t);
                let outward_normal = (p - self.center).unit();
                let front_face = ray.direction.dot(&outward_normal) < 0.0;
                let normal = if front_face { outward_normal } else { -outward_normal };

                // scattered ray
                let direction = self.material.scatter(ray.direction, normal, front_face);
                let depth = ray.depth + 1;
                let color = self.material.color(ray.direction, normal);

                let scatter = match direction {
                    Some(d) => Some(Ray{origin: p, direction: d, depth, color}),
                    None => None,
                };

                return Some(Hit{p, normal, t, front_face, scatter, color: ray.color * color})
            }
        }
        None
    }
}
