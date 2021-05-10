use wasm_bindgen::prelude::*;

#[macro_use]
mod utils;
mod ray;
use ray::{Color, Ray};

mod vec3;
use vec3::{Point, Vec3};
mod wobject;
use crate::wobject::World;


// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn trace_rays(width: u32, height: u32) -> Vec<u8> {
    // image
    let mut flat_image: Vec<u8> = Vec::new();
    let image_width = width as usize;
    let image_height= height as usize;
    let aspect_ratio = (image_width as f32) / (image_height as f32);

    // camera
    // TODO: move camera class to a separate module
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Point{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let center = Point{x: 0.0, y: 0.0, z: -focal_length};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 + center;

    // world
    let sphere = Box::new(wobject::Sphere{
        center: Point{x: 0.0, y: 0.0, z: -1.0},
        radius: 0.5,
    });

    let ground = Box::new(wobject::Sphere{
        center: Point{x: 0.0, y: -100.5, z: -1.0},
        radius: 100.0,
    });

    let mut world = World{
        wobjects: Vec::new(),
        closest_so_far: f32::MAX,
    };

    world.insert(sphere);
    world.insert(ground);

    // render
    for j in 0..image_height {
        for i in 0..image_width {
            let u = (i as f32) / (image_width as f32);
            let v = 1.0 - (j as f32) / (image_height as f32);
            let ray = Ray{
                origin: origin,
                direction: lower_left_corner + u*horizontal + v*vertical - origin,
            };
            world.closest_so_far = f32::MAX;
                
            // TODO: don't calculate background color unless not hit
            // TODO: only calculate normal if closest hit
            let mut color = ray.color();
            for item in world.wobjects.iter() {
                if let Some(hit) = item.hit(&ray, 0.0, world.closest_so_far) {
                    let normal = hit.normal;
                    color = Color{
                        r: (128. + 128.*normal.x) as u8,
                        g: (128. + 128.*normal.y) as u8,
                        b: (128. + 128.*normal.z) as u8,
                        a: 255,
                    };
                    world.closest_so_far = hit.t;
                }
            }
            flat_image.push(color.r);
            flat_image.push(color.g);
            flat_image.push(color.b);
            flat_image.push(color.a);
        }
    }
    return flat_image
}