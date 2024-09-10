use std::cmp::PartialEq;
use std::f64;
use std::ops::{Add, Div, Mul, Sub};

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    x: f64,
    y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Vec2 {
        Vec2 { x, y }
    }

    pub fn length(&self) -> f64 {
        self.length2().sqrt()
    }

    pub fn length2(&self) -> f64 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalized(&self) -> Vec2 {
        (*self) / self.length()
    }

    pub fn dot(&self, other: Vec2) -> f64 {
        (self.x * other.x) + (self.y * other.y)
    }
}

impl Add<Vec2> for Vec2 {
    type Output = Vec2;
    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Add<f64> for Vec2 {
    type Output = Vec2;
    fn add(self, num: f64) -> Vec2 {
        Vec2 {
            x: self.x + num,
            y: self.y + num,
        }
    }
}

impl Sub<Vec2> for Vec2 {
    type Output = Vec2;
    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Sub<f64> for Vec2 {
    type Output = Vec2;
    fn sub(self, num: f64) -> Vec2 {
        Vec2 {
            x: self.x - num,
            y: self.y - num,
        }
    }
}

impl Mul<Vec2> for Vec2 {
    type Output = Vec2;
    fn mul(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x * other.x,
            y: self.y * other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;
    fn mul(self, num: f64) -> Vec2 {
        Vec2 {
            x: self.x * num,
            y: self.y * num,
        }
    }
}

impl Div<Vec2> for Vec2 {
    type Output = Vec2;
    fn div(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x / other.x,
            y: self.y / other.y,
        }
    }
}

impl Div<f64> for Vec2 {
    type Output = Vec2;
    fn div(self, num: f64) -> Vec2 {
        Vec2 {
            x: self.x / num,
            y: self.y / num,
        }
    }
}

impl PartialEq for Vec2 {
    fn eq(&self, other: &Vec2) -> bool {
        self.x == other.x && self.y == other.y
    }
}
