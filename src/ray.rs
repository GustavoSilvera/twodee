use crate::math;

use math::Vec2;

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub o: Vec2,
    pub d: Vec2,
}

impl Ray {
    pub fn new(origin: Vec2, dir: Vec2) -> Ray {
        Ray { o: origin, d: dir }
    }

    // sample the ray at some t
    pub fn at(&self, t: f64) -> Vec2 {
        self.o + self.d * t
    }
}
