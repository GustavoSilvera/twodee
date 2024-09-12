use crate::math;
use crate::ray;
use crate::shape;

use math::Vec2;
use ray::Ray;
use shape::Circle;
use shape::Line;
use shape::Shape;
use shape::ShapeOps;

fn world() -> Vec<Shape> {
    let mut world: Vec<Shape> = Vec::new();
    world.push(Shape::Circle(Circle::new(Vec2::new(400.0, 300.0), 100.0)));

    let w0 = 10.0;
    let h0 = 10.0;
    let w1 = 800.0 - w0;
    let h1 = 600.0 - h0;

    // add walls
    world.push(Shape::Line(Line::new(Vec2::new(w0, h0), Vec2::new(w0, h1)))); //   Left wall
    world.push(Shape::Line(Line::new(Vec2::new(w0, h0), Vec2::new(w1, h0)))); //    Top wall
    world.push(Shape::Line(Line::new(Vec2::new(w1, h1), Vec2::new(w1, h0)))); //  Right wall
    world.push(Shape::Line(Line::new(Vec2::new(w1, h1), Vec2::new(w0, h1)))); // Bottom wall

    world
}

/// TODO: use sRGB type
fn ray_query((x, y): (f64, f64), world: &Vec<Shape>) -> [f64; 3] {
    for shape in world {
        if shape.is_on_surface(Vec2::new(x, y)) {
            return [1.0, 0.0, 0.0];
        }
    }
    [0.0, 0.0, 0.0]
}

pub fn render(pixels: &mut [u8]) {
    let (width, height) = (800, 600);

    let world = world();

    for y in 0..height {
        for x in 0..width {
            let rgb = ray_query((x as f64, y as f64), &world);

            let i = y * width + x;
            pixels[i * 3 + 0] = (rgb[0] * 255.0) as u8;
            pixels[i * 3 + 1] = (rgb[1] * 255.0) as u8;
            pixels[i * 3 + 2] = (rgb[2] * 255.0) as u8;
        }
    }
}
