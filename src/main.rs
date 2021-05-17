use std::env;
use std::path::Path;

use clap::{Arg, App};


use wasm_ray::trace_rays_parallel;


fn main() {
    let matches = App::new("My Super Program")
                          .version("0.1.0")
                          .author("Ashley A. <aganders3@gmail.com>")
                          .about("A simple ray-tracer")
                          .arg(Arg::with_name("width")
                               .help("image width")
                               .index(1))
                          .arg(Arg::with_name("height")
                               .help("image height")
                               .index(2))
                          .arg(Arg::with_name("aa")
                               .help("anti-aliasing factor")
                               .index(3))
                          .arg(Arg::with_name("fname")
                               .help("output file name")
                               .index(4))
                          .get_matches();

    let width = matches.value_of("width").unwrap_or("800").parse::<u32>().unwrap_or(800);
    let height = matches.value_of("height").unwrap_or("450").parse::<u32>().unwrap_or(450);
    let aa = matches.value_of("aa").unwrap_or("1").parse::<u8>().unwrap_or(1);
    let fname = matches.value_of("fname").unwrap_or("image.png");

    println!("Calling trace_rays_()");

    let im = trace_rays_parallel(width, height, aa);

    let result = image::save_buffer(&Path::new(fname), &im.image, width, height, image::ColorType::Rgba8);

    match result {
        Ok(_) => {
            println!("Nice! Check out your image at {}", fname);
            std::process::exit(0);
        }
        Err(msg) => {
            println!("Oh no! Something went wrong saving your image: {}", msg);
            std::process::exit(1);
        }
    }
}