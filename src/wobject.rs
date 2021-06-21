use itertools::izip;

use crate::material::Material;
use crate::ray::{Ray, Color};
use crate::vec3::{Point, Vec3};

// TODO: restructure to fit AABB as well, where we don't want to calculate a normal or color
// might just be able to make everything except p and t optional?
// or move them to some nested struct...
pub struct Hit {
    pub p: Point,
    pub t: f32,
    pub normal: Vec3,
    pub front_face: bool,
    pub scatter: Option<Ray>,
    pub color: Color,
}

pub trait Wobject {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Hit>;
    fn bb(&self) -> AxisAlignedBoundingBox;
}

#[derive(Debug)]
pub struct AxisAlignedBoundingBox {
    pub min: Point,
    pub max: Point,
}

impl AxisAlignedBoundingBox {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> bool {
        for (d, o, min, max) in izip!(ray.direction.into_iter(), ray.origin.into_iter(), self.min.into_iter(), self.max.into_iter())
        {
            let inv = 1.0 / d;
            let mut t0 = inv * (min - o);
            let mut t1 = inv * (max - o);
            if inv < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min = t0.max(t_min);
            let t_max = t1.min(t_max);
            if t_max <= t_min {
                return false
            }
        }
        true
    }

    fn from_wobjects(wobjects: Vec<impl Wobject>) -> Self {
        let mut min = Point{x: f32::INFINITY, y: f32::INFINITY, z: f32::INFINITY};
        let mut max = Point{x: -f32::INFINITY, y: -f32::INFINITY, z: -f32::INFINITY};

        for wobject in wobjects {
            let bb = wobject.bb();
            min.x = min.x.min(bb.min.x);
            min.y = min.y.min(bb.min.y);
            min.z = min.z.min(bb.min.z);

            max.x = max.x.max(bb.max.x);
            max.y = max.y.max(bb.max.y);
            max.z = max.z.max(bb.max.z);
        }

        AxisAlignedBoundingBox{min, max}
    }
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

            // scattered ray
            let direction = self.material.scatter(ray.direction, normal, front_face);
            let color = self.material.color(normal);
            let depth = ray.depth + 1;

            let scatter = direction.map(
                |d| Ray{origin: p, direction: d, depth, color: ray.color * color}
            );

            return Some(Hit{p, t, normal, front_face, scatter, color});
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
    fn aabb_from_sphere_list() {
        let spheres = vec!(
            Sphere {
                center: Point{x: 0.0, y: 0.0, z: -2.0},
                radius: 1.0,
                material: Material::NormalView,
            },
            Sphere {
                center: Point{x: 0.0, y: 1.0, z: -2.0},
                radius: 1.0,
                material: Material::NormalView,
            },
        );

        let bb = AxisAlignedBoundingBox::from_wobjects(spheres);

        assert!(
            bb.min == Point{x: -1.0, y: -1.0, z: -3.0} && bb.max == Point{x: 1.0, y: 2.0, z: -1.0}
        );
    }

    #[test]
    fn hit_aabb() {
        let sphere = Sphere {
            center: Point{x: 0.0, y: 0.0, z: -2.0},
            radius: 1.0,
            material: Material::NormalView,
        };

        let bb = sphere.bb();

        let ray = Ray{
            origin: Point{x: 0.0, y: 0.0, z: 0.0},
            direction: Vec3{x: 0.0, y: 0.0, z: -1.0},
            depth: 0,
            color: Color{r: 1.0, g: 1.0, b: 1.0},
        };

        assert!(bb.hit(&ray, 0.0, f32::INFINITY));
    }

    #[test]
    fn miss_aabb() {
        let sphere = Sphere {
            center: Point{x: 0.0, y: 0.0, z: -2.0},
            radius: 1.0,
            material: Material::NormalView,
        };

        let bb = sphere.bb();

        let ray = Ray{
            origin: Point{x: 0.0, y: 0.0, z: 0.0},
            direction: Vec3{x: 0.0, y: 0.0, z: 1.0},
            depth: 0,
            color: Color{r: 1.0, g: 1.0, b: 1.0},
        };

        assert!(!bb.hit(&ray, 0.0, f32::INFINITY));
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
