use rand::prelude::*;

use std::collections::VecDeque;

use crate::ray::{Color, Ray, blend};
use crate::vec3::{Vec3, Point};
use crate::wobject::Wobject;

pub struct Camera {
    image_height: usize,
    image_width: usize,

    origin: Point,
    horizontal: Vec3,
    vertical: Vec3,
    lower_left_corner: Point,

    anti_aliasing: u8,
    max_bounces: u8,
}

impl Camera {
    pub fn new(image_height: usize, image_width: usize, anti_aliasing: u8) -> Camera{
        let focal_length = 1.0;
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
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            anti_aliasing,
            max_bounces: 32,
        }
    }

    pub fn get_color(&self, world: &[Box<dyn Wobject + Send + Sync>], i: usize, j: usize) -> Color {
        let mut rays = self.get_aa_rays(i as f32, j as f32);
        let mut colors = Vec::with_capacity(self.anti_aliasing as usize);

        while !rays.is_empty() {
            if let Some(ray) = rays.pop_front() {
                if ray.depth > self.max_bounces {
                    colors.push(Color{r: 0.0, g: 0.0, b: 0.0});
                    continue;
                }

                let mut color = ray.background_color();
                let mut closest_so_far = f32::INFINITY;
                let mut scatter: Option<Ray> = None;
                for item in world.iter() {
                    if let Some(hit) = item.hit(&ray, 0.001, closest_so_far) {
                        closest_so_far = hit.t;
                        color = hit.color;
                        scatter = hit.scatter;
                    }
                }

                if let Some(new_ray) = scatter {
                    // hit and scattered
                    rays.push_back(new_ray);
                } else {
                    // absorbed or hit background
                    colors.push(ray.color * color);
                }
            }
        }
        blend(colors)
    }

    fn u_v_from_i_j(&self, i: f32, j: f32) -> [f32; 2] {
        [
            (i as f32) / (self.image_width as f32),
            1.0 - (j as f32) / (self.image_height as f32),
        ]
    }

    pub fn get_ray(&self, i: f32, j: f32) -> Ray {
        let [u, v] = self.u_v_from_i_j(i, j);
        Ray {
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
            depth: 0,
            color: Color{r: 1.0, g: 1.0, b: 1.0},
        }
    }

    pub fn get_aa_rays(&self, i: f32, j: f32) -> VecDeque<Ray> {
        let mut rays: VecDeque<Ray> = VecDeque::new();

        if self.anti_aliasing == 1 {
            rays.push_back(self.get_ray(i, j))
        } else {
            let mut rng = rand::thread_rng();
            for _ in 0..self.anti_aliasing {
                rays.push_back(self.get_ray(
                    i + rng.gen::<f32>(),
                    j + rng.gen::<f32>(),
                ))
            }
        }
        rays
    }
}
