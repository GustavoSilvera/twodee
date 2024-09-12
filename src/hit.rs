use crate::math;
use crate::ray;

use math::Vec2;
use ray::Ray;

pub struct Hit {
    pub t: f64,
    pub pos: Vec2,
    pub normal: Vec2,
}

impl Hit {
    pub fn new(t: f64, pt: Vec2, norm: Vec2) -> Hit {
        Hit {
            t: t,
            pos: pt,
            normal: norm,
        }
    }
}

pub trait Hittable {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<Hit>;
}
