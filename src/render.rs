use crate::math;

use math::Vec2;

struct Shape {
    posn: Vec2,
    radius: f64,
}

fn world() -> Vec<Shape> {
    let world: Vec<Shape> = Vec::new();
    world
}

pub fn render(pixels: &mut [u8]) {
    let (width, height) = (800, 600);

    let world = world();

    for y in 0..height {
        for x in 0..width {
            let rgb = [0.0, 0.0, 0.0];
            let i = y * width + x;
            pixels[i * 3 + 0] = (rgb[0] * 255.0) as u8;
            pixels[i * 3 + 1] = (rgb[1] * 255.0) as u8;
            pixels[i * 3 + 2] = (rgb[2] * 255.0) as u8;
        }
    }
}
