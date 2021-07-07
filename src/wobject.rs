use crate::aabb::AxisAlignedBoundingBox;
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

pub struct Sphere {
    pub center: Point,
    pub radius: f32,
    pub material: Material,
}

pub enum Elemental {
    Sphere(Sphere),
}

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
