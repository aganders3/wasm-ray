use wasm_bindgen::prelude::*;

#[macro_use]
mod utils;
mod ray;
use ray::{Color, Ray};

mod vec3;
use vec3::{Point, Vec3};
mod wobject;
use wobject::World;
mod camera;
use camera::Camera;
mod image;
use image::Image;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn trace_rays(width: u32, height: u32) -> *const u8 {
    // image
    let image_width = width as usize;
    let image_height= height as usize;

    let mut image = Image::new(image_height, image_width);

    // camera
    let cam = Camera::new(image_height, image_width, 1.0);

    // world
    let sphere = Box::new(wobject::Sphere {
        center: Point{x: 0.0, y: 0.0, z: -1.0},
        radius: 0.5,
    });

    let ground = Box::new(wobject::Sphere {
        center: Point{x: 0.0, y: -100.5, z: -1.0},
        radius: 100.0,
    });

    let mut world = World {
        wobjects: Vec::new(),
        closest_so_far: f32::MAX,
    };

    world.insert(sphere);
    world.insert(ground);

    // render
    for j in 0..image_height {
        for i in 0..image_width {
            let ray = cam.get_ray(i as f32, j as f32);
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
            image.write_color(i, j, color);
        }
    }
    image.image.as_ptr()
}