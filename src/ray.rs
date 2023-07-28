use crate::vec3::*;

#[derive(Copy, Clone, Default)]
pub struct Ray {
    pub origin: Point3,
    pub direction: Vec3
}

impl Ray{
    pub fn at(self: &Ray, t: f64) -> Point3 {
        let r: Vec3 = t * self.direction;
        r + self.origin
    }
}