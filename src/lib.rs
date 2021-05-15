use wasm_bindgen::prelude::*;

#[macro_use]
mod camera;
mod image;
mod ray;
mod utils;
mod vec3;
mod wobject;

use camera::Camera;
use image::Image;
use vec3::Point;
use wobject::World;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn trace_rays(width: u32, height: u32, aa: u8) -> *const u8 {
    // image
    let image_width = width as usize;
    let image_height= height as usize;

    let mut image = Image::new(image_height, image_width);

    // camera
    let cam = Camera::new(image_height, image_width, aa);

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
    };

    world.insert(sphere);
    world.insert(ground);

    // render
    for j in 0..image_height {
        for i in 0..image_width {
            let color = cam.get_color(&world, i, j);
            image.write_color(i, j, color);
        }
    }

    image.image.as_ptr()
}