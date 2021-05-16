use wasm_bindgen::prelude::*;
use rayon::prelude::*;

use std::path::Path;

#[macro_use]
mod utils;

mod camera;
mod im;
mod ray;
mod vec3;
mod wobject;

use camera::Camera;
use im::Image;
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
    }) as Box<dyn wobject::Wobject + Send + Sync>;

    let ground = Box::new(wobject::Sphere {
        center: Point{x: 0.0, y: -100.5, z: -1.0},
        radius: 100.0,
    }) as Box<dyn wobject::Wobject + Send + Sync>;

    // let mut world = World {
    //     wobjects: Vec::new(),
    // };
    let mut world = Vec::new();

    world.push(sphere);
    world.push(ground);

    // render
    for j in 0..image_height {
        for i in 0..image_width {
            let color = cam.get_color(&world, i, j);
            image.write_color(i, j, color);
        }
    }

    image.image.as_ptr()
}

pub fn trace_rays_to_image(width: u32, height: u32, aa: u8) {
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
    }) as Box<dyn wobject::Wobject + Send + Sync>;

    let ground = Box::new(wobject::Sphere {
        center: Point{x: 0.0, y: -100.5, z: -1.0},
        radius: 100.0,
    }) as Box<dyn wobject::Wobject + Send + Sync>;

    let mut world = Vec::new();

    world.push(sphere);
    world.push(ground);

    // render
    image.image.par_chunks_mut(4 * image_width).enumerate().for_each(
        |(j, row)| {
            for i in 0..image_width {
                let color = cam.get_color(&world, i, j);
                row[4*i + 0] = color.r;
                row[4*i + 1] = color.g;
                row[4*i + 2] = color.b;
                row[4*i + 3] = color.a;
            }
        }
    );

    image::save_buffer(&Path::new("image.png"), &image.image, width, height, image::ColorType::Rgba8);
}