use rand::prelude::*;

use crate::aabb::AxisAlignedBoundingBox;
use crate::material::Material;
use crate::ray::{Ray, Color};
use crate::vec3::Point;

// TODO: restructure hit to have an optional Interaction
// to fit AABB as well, where we don't want to calculate scatter or color
pub struct Interaction {
    pub scatter: Option<Ray>,
    pub color: Color,
}

/// A `Hit` struct provides information about a Ray-Wobject intersection
#[derive(Clone, Copy, Debug)]
pub struct Hit {
    pub p: Point,
    pub t: f32,
    // pub interaction: Option<Interaction>,
    pub scatter: Option<Ray>,
    pub color: Color,
}

/// A `Wobject` is a World-object, anything that can be hit by and interact with a ray
pub trait Wobject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;

    // TODO: this might be more useful in a separate trait
    fn bb(&self) -> AxisAlignedBoundingBox;
}

/// This enum contains all basic/irreducible Wobjects
#[derive(Debug,Clone,Copy)]
pub enum Elemental {
    Sphere(Sphere),
}

impl Elemental {
    fn center(&self) -> Point {
        match self {
            Self::Sphere(sphere) => sphere.center
        }
    }
}

pub fn sort_on_random_axis(wobjects: &mut [Elemental]) {
    let mut rng = rand::thread_rng();
    let axis = rng.gen::<f32>();
    if axis < 0.333 {
        wobjects.sort_by(|a, b| a.center().x.partial_cmp(&b.center().x).unwrap());
    } else if axis < 0.666 {
        wobjects.sort_by(|a, b| a.center().y.partial_cmp(&b.center().y).unwrap());
    } else {
        wobjects.sort_by(|a, b| a.center().z.partial_cmp(&b.center().z).unwrap());
    }
}

/// Generally these methods will dispatch to the individual Wobject structs
impl Wobject for Elemental {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit> {
        match self {
            Self::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
        }
    }

    fn bb(&self) -> AxisAlignedBoundingBox {
        match self {
            Self::Sphere(sphere) => sphere.bb(),
        }
    }
}

#[derive(Debug,Clone,Copy)]
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

            // scattered ray
            let direction = self.material.scatter(ray.direction, normal, front_face);
            let color = self.material.color(normal);
            let depth = ray.depth + 1;

            let scatter = direction.map(
                // 0.001 * d offset is to reduce shadow acne
                |d| Ray{origin: p + 0.001 * d, direction: d, depth, color: ray.color * color}
            );

            return Some(Hit{p, t, scatter, color});
        }
        None
    }

    fn bb(&self) -> AxisAlignedBoundingBox {
        AxisAlignedBoundingBox {
            min: Point{x: self.center.x - self.radius, y: self.center.y - self.radius, z: self.center.z - self.radius},
            max: Point{x: self.center.x + self.radius, y: self.center.y + self.radius, z: self.center.z + self.radius},
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vec3::Vec3;

    #[test]
    fn aabb_from_sphere() {
        let sphere = Sphere {
            center: Point{x: 0.0, y: 0.0, z: -2.0},
            radius: 1.0,
            material: Material::NormalView,
        };

        let bb = sphere.bb();

        assert!(
            bb.min == Point{x: -1.0, y: -1.0, z: -3.0} && bb.max == Point{x: 1.0, y: 1.0, z: -1.0}
        );
    }

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
