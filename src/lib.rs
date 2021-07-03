use rand::prelude::*;
use rayon::prelude::*;

mod aabb;
mod camera;
mod im;
pub mod material;
mod ray;
mod vec3;
mod wobject;

use camera::Camera;
use im::Image;
use material::Material;
use ray::Color;
use vec3::{Point, Vec3};

pub fn trace_rays(width: u32, height: u32, aa: u32) -> Image {
    // image
    let image_width = width as usize;
    let image_height= height as usize;

    let mut image = Image::new(image_height, image_width);

    // camera
    // let cam = Camera::new(90.0, image_height, image_width, aa);
    let cam = Camera::new(
        Point{x: -2.0, y: -2.0, z: 1.0},
        Point{x: 0.0, y: 0.0, z: -1.0},
        Vec3{x: 0.0, y: 1.0, z: 0.0},
        0.1, 10.0,
        90.0, image_height, image_width, aa,
    );

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

pub fn trace_rays_parallel(width: u32, height: u32, aa: u32) -> Image {
    // image
    let image_width = width as usize;
    let image_height= height as usize;

    let mut image = Image::new(image_height, image_width);

    // camera
    // camera for scene 18
    /*
    let cam = Camera::new(
        Point{x: -2.0, y: 2.0, z: 1.0},
        Point{x: 0.0, y: 0.0, z: -1.0},
        Vec3{x: 0.0, y: 1.0, z: 0.0},
        90.0, image_height, image_width, aa,
    );
    */
    // camera for cover scene
    let cam = Camera::new(
        Point{x: 13.0, y: 2.0, z: 3.0},
        Point{x: 0.0, y: 0.0, z: 0.0},
        Vec3{x: 0.0, y: 1.0, z: 0.0},
        0.1, 10.0,
        20.0, image_height, image_width, aa,
    );

    // world
    // println!("Generating the world...");
    // let world = scene_14();
    // let world = scene_18();
    let world = cover_scene();

    // render
    image.image.par_chunks_mut(3 * image_width).enumerate().for_each(
        |(j, row)| {
            // println!("Tracing row {}...", j);
            for i in 0..image_width {
                let color = cam.get_color(&world, i, j);
                row[3*i + 0] = (color.r.sqrt() * 256.) as u8;
                row[3*i + 1] = (color.g.sqrt() * 256.) as u8;
                row[3*i + 2] = (color.b.sqrt() * 256.) as u8;
            }
        }
    );

    image
}

pub fn empty() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![]
}

pub fn scene_10() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![
        // ground
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: Material::Lambertian {
                color: Color{r: 0.5, g: 0.5, b: 0.5},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // center
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Lambertian {
                color: Color{r: 0.5, g: 0.5, b: 0.5},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
    ]
}

pub fn scene_11() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![
        // ground
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: Material::Lambertian {
                color: Color{r: 0.8, g: 0.8, b: 0.0},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // left
        Box::new(wobject::Sphere {
            center: Point{x: -1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Metal {
                color: Color{r: 0.8, g: 0.8, b: 0.8},
                fuzz: 0.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // center
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Lambertian {
                color: Color{r: 0.7, g: 0.3, b: 0.3},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // right
        Box::new(wobject::Sphere {
            center: Point{x: 1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Metal {
                color: Color{r: 0.8, g: 0.6, b: 0.2},
                fuzz: 0.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
     ]
}

pub fn scene_12() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![
        // ground
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: Material::Lambertian {
                color: Color{r: 0.8, g: 0.8, b: 0.0},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // left
        Box::new(wobject::Sphere {
            center: Point{x: -1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Metal {
                color: Color{r: 0.8, g: 0.8, b: 0.8},
                fuzz: 0.3,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // center
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Lambertian {
                color: Color{r: 0.7, g: 0.3, b: 0.3},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // right
        Box::new(wobject::Sphere {
            center: Point{x: 1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Metal {
                color: Color{r: 0.8, g: 0.6, b: 0.2},
                fuzz: 1.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
     ]
}

pub fn scene_14() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![
        // ground
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: Material::Lambertian {
                color: Color{r: 0.8, g: 0.8, b: 0.0},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // left
        Box::new(wobject::Sphere {
            center: Point{x: -1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Dielectric {
                color: Color{r: 1.0, g: 1.0, b: 1.0},
                eta: 1.5,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // center
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Dielectric {
                color: Color{r: 1.0, g: 1.0, b: 1.0},
                eta: 1.5,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // right
        Box::new(wobject::Sphere {
            center: Point{x: 1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Metal {
                color: Color{r: 0.8, g: 0.6, b: 0.2},
                fuzz: 1.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
     ]
}

pub fn scene_18() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![
        // ground
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -100.5, z: -1.0},
            radius: 100.0,
            material: Material::Lambertian {
                color: Color{r: 0.8, g: 0.8, b: 0.0},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // left
        Box::new(wobject::Sphere {
            center: Point{x: -1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Dielectric {
                color: Color{r: 1.0, g: 1.0, b: 1.0},
                eta: 1.5,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // center
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Lambertian {
                color: Color{r: 0.1, g: 0.2, b: 0.5},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        // right
        Box::new(wobject::Sphere {
            center: Point{x: 1.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Metal {
                color: Color{r: 0.8, g: 0.6, b: 0.2},
                fuzz: 0.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
     ]
}

fn scene() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    vec![
        // ground
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -1000.5, z: 1.0},
            radius: 1000.0,
            material: Material::Lambertian {
                color: Color{r: 0.5, g: 0.5, b: 0.5},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: 0.0, z: -1.0},
            radius: 0.5,
            material: Material::Lambertian {
                color: Color{r: 0.5, g: 0.5, b: 0.5},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
     ]
}

fn cover_scene() -> Vec<Box<dyn wobject::Wobject + Send + Sync>> {
    let mut world = vec![
        // ground
        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: -1000.0, z: 0.0},
            radius: 1000.0,
            material: Material::Lambertian {
                color: Color{r: 0.5, g: 0.5, b: 0.5},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        Box::new(wobject::Sphere {
            center: Point{x: -4.0, y: 1.0, z: 0.0},
            radius: 1.0,
            material: Material::Lambertian {
                color: Color{r: 0.4, g: 0.2, b: 0.1},
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        Box::new(wobject::Sphere {
            center: Point{x: 0.0, y: 1.0, z: 0.0},
            radius: 1.0,
            material: Material::Dielectric {
                color: Color{r: 1.0, g: 1.0, b: 1.0},
                eta: 1.5,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,

        Box::new(wobject::Sphere {
            center: Point{x: 4.0, y: 1.0, z: 0.0},
            radius: 1.0,
            material: Material::Metal {
                color: Color{r: 0.7, g: 0.6, b: 0.5},
                fuzz: 0.0,
            },
        }) as Box<dyn wobject::Wobject + Send + Sync>,
    ];

    let mut rng = rand::thread_rng();
    for a in -11..11 {
        for b in -11..11 {
            let material = rng.gen::<f32>();

            let center = Point{
                x: a as f32 + 0.9*rng.gen::<f32>(),
                y: 0.2,
                z: b  as f32 + 0.9*rng.gen::<f32>(),
            };

            if (center - Point{x: 4.0, y: 0.2, z: 0.0}).length() <= 0.9 {
                continue;
            }

            let radius = 0.2;

            let sphere;
            if material < 0.8 {
                sphere = Box::new(wobject::Sphere {
                    center,
                    radius,
                    material: Material::Lambertian {
                        color: Color::random() * Color::random(),
                    },
                }) as Box<dyn wobject::Wobject + Send + Sync>;
            } else if material < 0.95 {
                sphere = Box::new(wobject::Sphere {
                    center,
                    radius,
                    material: Material::Metal {
                        color: Color::random(),
                        fuzz: rng.gen::<f32>(),
                    },
                }) as Box<dyn wobject::Wobject + Send + Sync>;
            } else {
                sphere = Box::new(wobject::Sphere {
                    center,
                    radius,
                    material: Material::Dielectric {
                        color: Color{r: 1.0, g: 1.0, b: 1.0},
                        eta: 1.5,
                    },
                }) as Box<dyn wobject::Wobject + Send + Sync>;
            }
            world.push(sphere);
        }
    }

    world
}
