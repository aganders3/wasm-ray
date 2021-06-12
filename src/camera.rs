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

    lens_radius: f32,

    anti_aliasing: u32,
    max_bounces: u8,
}

fn degrees_to_radians(degrees: f32) -> f32 {
    degrees * std::f32::consts::PI / 180.0
}

impl Camera {
    pub fn new(
        lookfrom: Point, lookat: Point, vup: Vec3,
        aperture: f32, focus_dist: f32,
        vfov: f32, image_height: usize, image_width: usize,
        anti_aliasing: u32
    ) -> Camera {
        let aspect_ratio = (image_width as f32) / (image_height as f32);

        let h = degrees_to_radians(vfov / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;

        Camera {
            image_height,
            image_width,
            origin,
            horizontal,
            vertical,
            lower_left_corner,
            lens_radius,
            anti_aliasing,
            max_bounces: 16,
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
        // TODO: only do this if lens_radius > 0
        let rd = self.lens_radius * Vec3::random_in_unit_disk();
        let offset = self.horizontal.unit() * rd.x + self.vertical.unit() * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
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
                rays.push_back(
                    self.get_ray(
                        i + rng.gen::<f32>(),
                        j + rng.gen::<f32>(),
                    )
                )
            }
        }
        rays
    }
}
