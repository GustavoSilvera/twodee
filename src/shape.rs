use crate::hit;
use crate::math;
use crate::ray;

use hit::Hit;
use hit::Hittable;
use math::Vec2;
use ray::Ray;

pub enum Shape {
    Circle(Circle),
    Line(Line),
}

pub trait ShapeOps {
    fn intersects(&self, pos: Vec2) -> bool;
    fn is_on_surface(&self, pos: Vec2) -> bool;
    fn surface_ray(&self, pos: Vec2) -> Option<Ray>;
}

impl ShapeOps for Shape {
    fn intersects(&self, pos: Vec2) -> bool {
        match self {
            Shape::Circle(c) => c.intersects(pos),
            Shape::Line(l) => l.intersects(pos),
        }
    }
    fn is_on_surface(&self, pos: Vec2) -> bool {
        match self {
            Shape::Circle(c) => c.is_on_surface(pos),
            Shape::Line(l) => l.is_on_surface(pos),
        }
    }
    fn surface_ray(&self, pos: Vec2) -> Option<Ray> {
        match self {
            Shape::Circle(c) => c.surface_ray(pos),
            Shape::Line(l) => l.surface_ray(pos),
        }
    }
}

impl Hittable for Shape {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        match self {
            Shape::Circle(c) => c.hit(ray, t_min, t_max),
            Shape::Line(l) => l.hit(ray, t_min, t_max),
        }
    }
}

pub struct Circle {
    center: Vec2,
    radius: f64,
}

impl Circle {
    pub fn new(pos: Vec2, rad: f64) -> Circle {
        Circle {
            center: pos,
            radius: rad,
        }
    }
}

impl ShapeOps for Circle {
    fn intersects(&self, pos: Vec2) -> bool {
        (pos - self.center).length2() < self.radius * self.radius
    }

    fn is_on_surface(&self, pos: Vec2) -> bool {
        let d2 = (pos - self.center).length2();
        let r2 = self.radius * self.radius;
        let eps = d2 * 0.01;

        d2 - eps < r2 && d2 + eps > r2
    }

    fn surface_ray(&self, pos: Vec2) -> Option<Ray> {
        if !self.is_on_surface(pos) {
            return None;
        }

        let o = pos;
        let n = (pos - self.center).normalized();

        Some(Ray { o: o, d: n })
    }
}

impl Hittable for Circle {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // TODO
        None
    }
}

pub struct Line {
    a: Vec2,
    b: Vec2,
}

impl Line {
    pub fn new(p1: Vec2, p2: Vec2) -> Line {
        Line { a: p1, b: p2 }
    }
}

impl ShapeOps for Line {
    fn intersects(&self, pos: Vec2) -> bool {
        let d1 = (pos - self.a).length();
        let d2 = (pos - self.b).length();

        d1 + d2 == (self.a - self.b).length()
    }

    fn is_on_surface(&self, pos: Vec2) -> bool {
        self.intersects(pos)
    }

    fn surface_ray(&self, pos: Vec2) -> Option<Ray> {
        if !self.is_on_surface(pos) {
            return None;
        }

        let o = pos;
        let n = (self.a - self.b).perp().normalized(); // TODO: try both ends

        Some(Ray::new(o, n))
    }
}

impl Hittable for Line {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // TODO
        None
    }
}
