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
}

pub struct Circle {
    posn: Vec2,
    radius: f64,
}

impl Circle {
    pub fn new(pos: Vec2, rad: f64) -> Circle {
        Circle {
            posn: pos,
            radius: rad,
        }
    }
}

impl ShapeOps for Circle {
    fn intersects(&self, pos: Vec2) -> bool {
        (pos - self.posn).length2() < self.radius * self.radius
    }

    fn is_on_surface(&self, pos: Vec2) -> bool {
        let d2 = (pos - self.posn).length2();
        let r2 = self.radius * self.radius;
        let eps = d2 * 0.01;

        d2 - eps < r2 && d2 + eps > r2
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
        let vec = self.a - self.b;
        let pvec = pos - self.b;

        vec.dot(pvec) < 0.99
    }

    fn is_on_surface(&self, pos: Vec2) -> bool {
        self.intersects(pos)
    }
}

impl Hittable for Line {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // TODO
        None
    }
}
