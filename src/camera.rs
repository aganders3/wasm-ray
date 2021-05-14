use rand::prelude::*;

use crate::ray::Ray;
use crate::vec3::{Vec3, Point};

pub struct Camera {
    pub image_height: usize,
    pub image_width: usize,
    pub aspect_ratio: f32,

    viewport_height: f32,
    viewport_width: f32,

    focal_length: f32,
    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    center: Point,
    lower_left_corner: Point,
}

impl Camera {
    pub fn new(image_height: usize, image_width: usize, focal_length: f32) -> Camera{
        let aspect_ratio = (image_width as f32) / (image_height as f32);
        let origin = Point{x: 0.0, y: 0.0, z: 0.0};
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
        let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
        let center = Point{x: 0.0, y: 0.0, z: -focal_length};
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 + center;

        Camera {
            image_height,
            image_width,
            aspect_ratio,
            viewport_height,
            viewport_width,
            focal_length,
            origin,
            horizontal,
            vertical,
            center,
            lower_left_corner,
        }
    }

    pub fn u_v_from_i_j(&self, i: f32, j: f32) -> [f32; 2] {
        [
            (i as f32) / (self.image_width as f32),
            1.0 - (j as f32) / (self.image_height as f32),
        ]
    }

    pub fn get_ray(&self, i: f32, j: f32) -> Ray {
        let [u, v] = self.u_v_from_i_j(i, j);
        Ray{
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }

    pub fn get_aa_rays(&self, i: f32, j: f32, n: u8) -> Vec<Ray> {
        let mut rays: Vec<Ray> = Vec::new();
        let mut rng = rand::thread_rng();
        for _ in 0..n {
            let i_ = i + rng.gen::<f32>();
            let j_ = j + rng.gen::<f32>();
            rays.push(self.get_ray(i_, j_))
        }
        rays
    }
}