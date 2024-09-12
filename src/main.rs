mod hit;
mod math;
mod ray;
mod render;
mod utils;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let width = 800;
    let height = 600;
    println!("Starting 2D raytracer @ {}x{}", width, height);

    let mut pixels = vec![0; width * height * 3];

    if args.len() == 1 {
        println!("Pass in output file as argument!");
        return;
    }

    render::render(&mut pixels);

    utils::write_image(&args[1], &pixels, (width, height)).expect("Error writing image");
}
