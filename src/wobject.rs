use crate::ray::{Ray, Color};
use crate::vec3::{Vec3, Point};

pub struct Hit {
    pub p: Point,
    pub normal: Vec3,
    pub t: f32,
    pub attenuation: f32,
    pub scatter: Option<Ray>,
    pub color: Option<Color>,
}

pub trait Wobject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
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
                let p = ray.at(t);
                let normal = (p - self.center).unit();
                return Some(Hit{
                    p,
                    normal,
                    t,
                    attenuation: 0.2,
                    scatter: Some(Ray{
                        origin: p,
                        // metal
                        direction: ray.direction - 2.0 * ray.direction.dot(&normal) * normal,
                    }),
                    color: None,
                    // color: Some(Color::from_normal(normal)),
                    // color: Some(Color{r: 32, g: 32, b: 128, a: 255}),
                })
            }
        }
        None
    }
}
