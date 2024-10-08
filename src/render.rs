use crate::hit;
use crate::math;
use crate::ray;
use crate::shape;

use hit::Hit;
use hit::Hittable;
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

fn hit_world(world: &Vec<Shape>, r: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
    let mut closest = t_max;
    let mut ret = None;
    for shape in world {
        if let Some(hit) = shape.hit(r, t_min, closest) {
            closest = hit.t;
            ret = Some(hit);
        }
    }
    ret
}

fn ray_color(ray: &Ray, world: &Vec<Shape>, depth: i32) -> [f64; 3] {
    if depth <= 0 {
        return [0.0, 0.0, 0.0];
    }
    let hit = hit_world(world, ray, 0.001, std::f64::MAX);
    match hit {
        Some(hit) => {
            let scattered_ray = Ray::new(hit.pos, hit.normal);
            let _target_col = ray_color(&scattered_ray, world, depth - 1);
            return [1.0, 0.0, 0.0];
        }
        None => {
            return [0.0, 0.0, 0.0];
        }
    }
}

/// TODO: use sRGB type
fn ray_query((x, y): (f64, f64), world: &Vec<Shape>) -> [f64; 3] {
    let bounces = 2;
    for shape in world {
        let r_opt = shape.surface_ray(Vec2::new(x, y));
        if !r_opt.is_none() {
            let r = r_opt.unwrap();
            return ray_color(&r, &world, bounces);
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
