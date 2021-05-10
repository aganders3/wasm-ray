use crate::ray::Ray;
use crate::vec3::{Vec3, Point};

pub struct Hit {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
}

pub trait Wobject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
}

pub struct World {
    pub wobjects: Vec<Box<dyn Wobject>>,
    pub closest_so_far: f32,
}

impl World {
    pub fn insert(&mut self, wobject: Box<dyn Wobject>) {
        self.wobjects.push(wobject);
    }
}

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
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
                return Some(Hit{
                    p: ray.at(t),
                    normal: (ray.at(t) - self.center).unit(),
                    t,
                })
            }
        }
        None
    }
}