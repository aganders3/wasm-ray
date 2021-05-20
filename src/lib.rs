use wasm_bindgen::prelude::*;
use rayon::prelude::*;

#[macro_use]
mod utils;

mod camera;
mod im;
mod material;
mod ray;
mod vec3;
mod wobject;

use camera::Camera;
use im::Image;
use material::Material;
use ray::Color;
use vec3::Point;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;


#[wasm_bindgen]
pub fn trace_rays_wasm(width: u32, height: u32, aa: u8) -> *const u8 {
    trace_rays(width, height, aa).image.as_ptr()
}

pub fn trace_rays(width: u32, height: u32, aa: u8) -> Image {
    // image
    let image_width = width as usize;
    let image_height= height as usize;

    let mut image = Image::new(image_height, image_width);

    // camera
    let cam = Camera::new(image_height, image_width, aa);

    // world
    let world = scene();

    // render
    for j in 0..image_height {
        for i in 0..image_width {
            let color = cam.get_color(&world, i, j);
            image.write_color(i, j, color);
        }
    }

    image
}

pub fn trace_rays_parallel(width: u32, height: u32, aa: u8) -> Image {
    // image
    let image_width = width as usize;
    let image_height= height as usize;

    let mut image = Image::new(image_height, image_width);

    // camera
    let cam = Camera::new(image_height, image_width, aa);

    // world
    let world = scene();

    // render
    image.image.par_chunks_mut(3 * image_width).enumerate().for_each(
        |(j, row)| {
            for i in 0..image_width {
                let color = cam.get_color(&world, i, j);
                row[3*i + 0] = (color.r * 255.) as u8;
                row[3*i + 1] = (color.g * 255.) as u8;
                row[3*i + 2] = (color.b * 255.) as u8;
            }
        }
    );

    image
}

fn scene() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![
        Box::new(wobject::Sphere {
            center: Point{x: -0.25, y: -0.3, z: -0.75},
            radius: 0.1,
            material: Material::Metal {
                color: Color{r: 0.5, g: 0.1, b: 0.5},
                fuzz: 0.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
        Box::new(wobject::Sphere {
            center: Point{x: -0.5, y: 0.3, z: -1.0},
            radius: 0.25,
            material: Material::Metal {
                color: Color{r: 0.1, g: 0.5, b: 0.5},
                fuzz: 1.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
        Box::new(wobject::Sphere {
            center: Point{x: 0.5, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Dielectric {
                color: Color{r: 1.0, g: 1.0, b: 1.0},
                eta: 3.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: Material::Lambertian {
                color: Color{r: 0.1, g: 0.1, b: 1.0},
                fuzz: 1.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
    ]
}
