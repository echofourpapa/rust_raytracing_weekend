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
    _w: Vec3,
    len_radius: f64
}

impl Camera {
    pub fn new(origin:Point3, look_at:Point3, up:Vec3, vfov:f64, aspect_ratio:f64, aperture: f64, focus_dist:f64) -> Camera {
        let theta: f64 = degrees_to_radians(vfov);
        let h: f64 = (theta/2.0).tan();

        let viewport_height: f64 = 2.0 * h;
        let viewport_width: f64 = aspect_ratio * viewport_height;

        let w: Vec3 = normalize(origin - look_at);
        let u: Vec3 = normalize(cross(&up, &w));
        let v: Vec3 = cross(&w, &u);
        
        let horizontal: Vec3 =  viewport_width * u * focus_dist;
        let vertical: Vec3 =  viewport_height * v * focus_dist;
        let lower_left_corner: Vec3 = origin - horizontal/2.0 - vertical/2.0 - w * focus_dist;

        let len_rad = aperture/2.0;
        
        Camera {
            origin: origin,
            lower_left_corner: lower_left_corner,
            horizontal: horizontal,
            vertical: vertical,
            u: u,
            v: v,
            _w: w,
            len_radius: len_rad
        }
    }

    pub fn get_ray(self, s:f64, t:f64) -> Ray {
        let rd: Vec3 = self.len_radius * random_in_unit_disk();
        let offset: Vec3 = self.u * rd.x() + self.v * rd.y();
        
        Ray{
            origin: self.origin + offset,
            direction: self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin - offset,
        }
    }
}