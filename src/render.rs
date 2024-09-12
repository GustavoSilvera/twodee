use crate::hit;
use crate::math;
use crate::ray;

use hit::Hit;
use hit::Hittable;
use math::Vec2;
use ray::Ray;

struct Shape {
    posn: Vec2,
    radius: f64,
}

impl Shape {
    pub fn new(pos: Vec2, rad: f64) -> Shape {
        Shape {
            posn: pos,
            radius: rad,
        }
    }
    pub fn intersects(&self, pos: Vec2) -> bool {
        (pos - self.posn).length2() <= self.radius * self.radius
    }
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // TODO
        None
    }
}

fn world() -> Vec<Shape> {
    let mut world: Vec<Shape> = Vec::new();
    world.push(Shape::new(Vec2::new(400.0, 300.0), 100.0));

    world
}

/// TODO: use sRGB type
fn ray_query((x, y): (f64, f64), world: &Vec<Shape>) -> [f64; 3] {
    for shape in world {
        if shape.intersects(Vec2::new(x, y)) {
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
