use crate::vec3::*;
use crate::ray::*;

#[derive(Copy, Clone, Default)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new() -> Camera {
        let aspect_ratio = 16.0/9.0;
        let viewport_height = 2.0;
        let viewport_width = aspect_ratio * viewport_height;
        let focal_length = 1.0;
        
        let origin = Point3{x:0.0, y:0.0, z:0.0};
        let horizontal = Vec3{x:viewport_width, y:0.0, z:0.0};
        let vertical = Vec3{x:0.0, y:viewport_height, z:0.0};
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3{x:0.0, y:0.0, z:focal_length};
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
        }
    }

    pub fn get_ray(self, u:f64, v:f64) -> Ray {
        Ray{
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin,
        }
    }
}