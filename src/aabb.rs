use itertools::izip;

use crate::ray::Ray;
use crate::vec3::Point;
use crate::wobject::Wobject;

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

pub struct BBTree {

}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::material::Material;
    use crate::ray::Color;
    use crate::vec3::Vec3;
    use crate::wobject::Sphere;

    #[test]
    fn aabb_from_list() {
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
}
