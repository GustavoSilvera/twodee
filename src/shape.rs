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
        // https://www.bluebill.net/circle_ray_intersection.html
        let u = self.center - ray.o;
        let u1 = ray.d * u.dot(ray.d);
        let u2 = u - u1;
        let d = u2.length();
        if d < self.radius {
            // 2 intersections
            let m = (self.radius * self.radius - d * d).sqrt();
            // let p1 = ray.o + u1 + m * ray.d; // further
            let p2 = ray.o + u1 - ray.d * m;
            let n = (p2 - self.center).normalized();
            let t = (p2 - ray.o).length();
            return Some(Hit {
                t: t,
                pos: p2,
                normal: n,
            });
        } else if d == self.radius {
            let p = ray.o + u1;
            let n = (p - self.center).normalized();
            let t = u1.length();
            return Some(Hit {
                t: t,
                pos: p,
                normal: n,
            });
        }
        // no intersection
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

    fn get_normals(&self, _pos: Vec2) -> (Vec2, Vec2) {
        (
            (self.a - self.b).perp().normalized(),
            (self.b - self.a).perp().normalized(),
        )
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
        let (n1, _n2) = self.get_normals(pos);
        let n = n1; // TODO: try both normals

        Some(Ray::new(o, n))
    }
}

impl Hittable for Line {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit> {
        // https://stackoverflow.com/a/565282
        // TODO: check vector lengths and normalizations?
        let p = self.a;
        let q = ray.o;
        let r = self.b - self.a;
        let s = ray.d;
        let rxs = r.cross2(s);
        if rxs == 0.0 {
            // two lines are collinear
            // todo consider two lines overlappinf or disjoint
            return None;
        }
        let t = (q - p).cross2(s) / rxs;
        let u = (q - p).cross2(r) / rxs;

        let opposite_dir = s.dot(r) < 0.0;
        let bounds01 = if opposite_dir { 1.0..0.0 } else { 0.0..1.0 };
        let raybounds = if opposite_dir {
            t_max..t_min
        } else {
            t_min..t_max
        };

        if bounds01.contains(&t) && raybounds.contains(&u) {
            // lines connect at p + tr == q + us
            let pos = ray.at(u);
            let (n1, n2) = self.get_normals(pos);
            // use the normal opposite direction of incoming ray
            let n = if n1.dot(ray.d) < 0.0 { n1 } else { n2 };
            return Some(Hit {
                t: u,
                pos: pos,
                normal: n,
            });
        }

        None
    }
}
