use crate::math::Float;
use crate::vec3::Vec3;

#[derive(Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub t: Float,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction,
            t: 0.0,
        }
    }

    pub fn new_at(origin: Vec3, direction: Vec3, t: Float) -> Self {
        Self {
            origin,
            direction,
            t,
        }
    }

    pub fn at(&self, t: Float) -> Vec3 {
        self.origin + self.direction * t
    }
}
