use wasm_ray::trace_rays_to_image;

fn main() {
    println!("Calling trace_rays()");

    let im = trace_rays_to_image(800, 450, 100);

}