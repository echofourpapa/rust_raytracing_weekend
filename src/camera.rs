use crate::vec3::*;
use crate::ray::*;
use crate::common::*;

#[derive(Copy, Clone, Default)]
pub struct Camera {
    origin: Point3,
    lower_left_corner: Point3,
    horizontal: Vec3,
    vertical: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    len_radius: f64
}

impl Camera {
    pub fn new(origin:Point3, look_at:Point3, up:Vec3, vfov:f64, aspect_ratio:f64, aperture: f64, focus_dist:f64) -> Camera {
        let theta = DegreesToRadians(vfov);
        let h = (theta/2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = aspect_ratio * viewport_height;

        let w = normalize(origin - look_at);
        let u = normalize(cross(&up, &w));
        let v = cross(&w, &u);
        
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - focus_dist*w;
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            w: w,
            len_radius: aperture/2.0
        }
    }

    pub fn get_ray(self, u:f64, v:f64) -> Ray {
        let rd = self.len_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;
        Ray{
            origin: self.origin,
            direction: self.lower_left_corner + u * self.horizontal + v * self.vertical - self.origin - offset,
        }
    }
}