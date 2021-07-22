use itertools::izip;

use crate::ray::Ray;
use crate::vec3::Point;
use crate::wobject::{Elemental, Hit, Wobject, sort_on_random_axis};

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

    fn from_wobjects(wobjects: &[Elemental]) -> Self {
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

#[derive(Debug)]
pub struct BVHNode {
    bb: AxisAlignedBoundingBox,
    //left: Option<Box<BVHNode>>,
    //right: Option<Box<BVHNode>>,
    left: Box<Option<BVHNode>>,
    right: Box<Option<BVHNode>>,
    wobject: Option<Elemental>,
    // wobject: Option<i32>,
}

pub fn closer(hit1: Option<Hit>, hit2: Option<Hit>) -> Option<Hit> {
    if let Some(h1) = hit1 {
        if let Some(h2) = hit2 {
            if h1.t < h2.t {
                return hit1
            } else {
                return hit2
            }
        }
        return hit1
    }
    return hit2
}

impl BVHNode {
    pub fn closest_hit(&self, ray: &Ray, depth: u32) -> Option<Hit> {
        // println!("{}", depth);
        if !self.bb.hit(ray, 0.001, f32::INFINITY) {
            return None
        }

        if let Some(wobject) = self.wobject {
            return wobject.hit(ray, 0.001, f32::INFINITY)
        }

        let left_hit = match &*self.left {
            Some(node) => node.closest_hit(ray, depth+1),
            None => None
        };

        let right_hit = match &*self.right {
            Some(node) => node.closest_hit(ray, depth+1),
            None => None
        };

        return closer(left_hit, right_hit)
    }
}

pub fn bvh_tree_from(wobjects: &mut [Elemental])-> Option<BVHNode> {
    let n = wobjects.len();
    let mut left = None;
    let mut right = None;
    let mut wobject = None;
    if n > 1 {
        // TODO: sort along a random axis before dividing in half
        // wobjects.sort_by(|a, b| a.partial_cmp(b).unwrap());
        sort_on_random_axis(wobjects);
        left = bvh_tree_from(&mut wobjects[0..n/2]);
        right = bvh_tree_from(&mut wobjects[n/2..n]);
    } else if n == 1 {
        // wobject = Some(100);
        wobject = Some(wobjects[0]);
    } else {
        return None
    }

    let bb = AxisAlignedBoundingBox::from_wobjects(wobjects);
    println!("{:?}", bb);
    Some(BVHNode {
        bb,
        left: Box::new(left),
        right: Box::new(right),
        wobject,
    })
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
            Elemental::Sphere(
                Sphere {
                    center: Point{x: 0.0, y: 0.0, z: -2.0},
                    radius: 1.0,
                    material: Material::NormalView,
                }
            ),
            Elemental::Sphere(
                Sphere {
                    center: Point{x: 0.0, y: 1.0, z: -2.0},
                    radius: 1.0,
                    material: Material::NormalView,
                }
            ),
        );

        let bb = AxisAlignedBoundingBox::from_wobjects(&spheres[..]);

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
    fn bvh_from_list() {
        let spheres = vec!(
            Elemental::Sphere(
                Sphere {
                    center: Point{x: 0.0, y: 0.0, z: -2.0},
                    radius: 1.0,
                    material: Material::NormalView,
                }
            ),
            Elemental::Sphere(
                Sphere {
                    center: Point{x: 0.0, y: 1.0, z: -2.0},
                    radius: 1.0,
                    material: Material::NormalView,
                }
            ),
            Elemental::Sphere(
                Sphere {
                    center: Point{x: 0.0, y: 1.0, z: -2.0},
                    radius: 1.0,
                    material: Material::NormalView,
                }
            ),
        );
        let n = bvh_tree_from(&spheres);
        println!("{:?}", n);
    }
}
