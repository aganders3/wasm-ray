use wasm_bindgen::prelude::*;

#[macro_use]
mod utils;

mod vec3;
use vec3::Vec3;
type Point = Vec3;

#[derive(Clone, Copy, Debug)]
struct Color{r: u8, g: u8, b:u8, a: u8}

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

struct Ray {
    origin: Point,
    direction: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }

    pub fn color(&self) -> Color {
        let t = 0.5 * (self.direction.unit().y + 1.0);
        Color{
            r: ((1.0 - t) * 255. + t * 128.) as u8,
            g: ((1.0 - t) * 255. + t * 178.) as u8,
            b: 255,
            a: 255,
        }
    }

    // TODO: add fn 'hit' that takes a geometry object
}

fn hit_sphere(ray: &Ray, center: Point, radius: f32) -> f32 {
    let oc = ray.origin - center;
    let a = ray.direction.dot(&ray.direction);
    let b = 2.0 * oc.dot(&ray.direction);
    let c = oc.dot(&oc) - radius*radius;
    let discriminant = b * b - 4.0 * a * c;

    if discriminant < 0.0 {
        return -1.0
    } else {
        return (-b - discriminant.sqrt()) / (2.0 * a);
    }
}

#[wasm_bindgen]
pub fn trace_rays() -> Vec<u8> {
    // image
    let mut flat_image: Vec<u8> = Vec::new();
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = ((image_width as f32) / aspect_ratio) as usize;

    // camera
    // TODO: make a separate struct?
    let viewport_height = 2.0;
    let viewport_width = aspect_ratio * viewport_height;
    let focal_length = 1.0;
    let origin = Point{x: 0.0, y: 0.0, z: 0.0};
    let horizontal = Vec3{x: viewport_width, y: 0.0, z: 0.0};
    let vertical = Vec3{x: 0.0, y: viewport_height, z: 0.0};
    let center = Point{x: 0.0, y: 0.0, z: -focal_length};
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 + center;

    // render
    for j in 0..image_height {
        for i in 0..image_width {
            let u = (i as f32) / (image_width as f32);
            let v = 1.0 - (j as f32) / (image_height as f32);
            let ray = Ray{
                origin: origin,
                direction: lower_left_corner + u*horizontal + v*vertical - origin,
            };

            let color: Color;
            let t = hit_sphere(&ray, Point{x: 0.0, y: 0.0, z: -1.0}, 0.5);
            
            if t < 0.0 {
                color = ray.color();
            } else {
                let normal = (ray.at(t) - center).unit();
                color = Color{
                    r: (128. + 128.*normal.x) as u8,
                    g: (128. + 128.*normal.y) as u8,
                    b: (128. + 128.*normal.z) as u8,
                    a: 255,
                };
            }
            flat_image.push(color.r);
            flat_image.push(color.g);
            flat_image.push(color.b);
            flat_image.push(color.a);
            // log!("{} {} {:?}", i, j, col);
        }
    }
    return flat_image
}
