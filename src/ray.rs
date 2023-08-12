use std::fmt;

use crate::vec3::*;

#[derive(Copy, Clone, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray{

    pub fn new(origin: Point3, direction:Vec3, time:f64) -> Ray {
        Ray { 
            origin: origin, 
            direction: direction, 
            time: time
        }
    }

    pub fn at(self: &Ray, t: f64) -> Point3 {
        let r: Vec3 = t * self.direction;
        r + self.origin
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Ray(origin:{}, direction:{}, time:{})", self.origin, self.direction, self.time)
    }
}