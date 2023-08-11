use std::sync::Arc;

use crate::common::degrees_to_radians;
use crate::interval::*;
use crate::material::Material;
use crate::vec3::*;
use crate::ray::*;
use crate::aabb::*;

#[derive(Clone, Default)]
pub struct HitRecord {
    pub p:Point3,
    pub normal: Vec3,
    pub mat: Option<Arc<dyn Material + Sync>>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool
}

impl HitRecord {
    pub fn set_face_normal(self: &mut HitRecord, r: &Ray, outward_normal: &Vec3) {
        self.front_face = dot(&r.direction, outward_normal) < 0.0;
        self.normal = if self.front_face {*outward_normal} else {-(*outward_normal)};
    }
    
}

pub trait Hittable : Send {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> AABB;
}

#[derive(Default, Clone)]
pub struct Translate {
    pub object: Option<Arc<dyn Hittable + Sync>>,
    pub offset: Vec3,
    pub bbox: AABB
}

impl Translate {
    pub fn new(p: Arc<dyn Hittable + Sync>, displacement: &Vec3) -> Translate {
        let bbox: AABB = p.bounding_box() + *displacement;
        Translate { 
            object: Some(p), 
            offset: *displacement, 
            bbox: bbox
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r: Ray = Ray::new(r.origin - self.offset, r.direction, r.time);

        if ! self.object.as_ref().unwrap().hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;
        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

#[derive(Default, Clone)]
pub struct RotateY {
    pub object: Option<Arc<dyn Hittable + Sync>>,
    pub sin_theta: f64,
    pub cos_theta: f64,
    pub bbox: AABB
}

impl RotateY {
    pub fn new(p: Arc<dyn Hittable + Sync>, angle: f64) -> RotateY {

        let radians: f64 = degrees_to_radians(angle);
        let s_c: (f64, f64) = radians.sin_cos();
        let sin_theta: f64 = s_c.0;
        let cos_theta: f64 = s_c.1;

        let bbox: AABB = p.bounding_box();

        let mut min: Vec3 = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut max: Vec3 = Point3::new(-f64::INFINITY, -f64::INFINITY, -f64::INFINITY);

        for i in 0..2 {
            for j  in 0..2{
                for k in 0..2{
                    let x: f64= i as f64 * bbox.x.max +(1-i) as f64 * bbox.x.min;
                    let y: f64= j as f64 * bbox.x.max +(1-j) as f64 * bbox.x.min;
                    let z: f64= k as f64 * bbox.x.max +(1-k) as f64 * bbox.x.min;

                    let newx: f64 = cos_theta * x + sin_theta * z;
                    let newz: f64 = -sin_theta * x + cos_theta * z;

                    let tester: Vec3 = Vec3::new(newx, y, newz);
                    for c in 0..3 {
                        min[c as usize] = min[c as usize].min(tester[c as usize]);
                        max[c as usize] = max[c as usize].max(tester[c as usize]);
                    }
                }
            }
        }

        RotateY { 
            object: Some(p), 
            sin_theta: sin_theta,
            cos_theta: cos_theta, 
            bbox: AABB::new(&min, &max)
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut origin: Vec3 = r.origin;
        let mut direction: Vec3 = r.direction;

        origin[0] = self.cos_theta * r.origin[0] - self.sin_theta * r.origin[2];
        origin[2] = self.sin_theta * r.origin[0] + self.cos_theta * r.origin[2];

        direction[0] = self.cos_theta * r.direction[0] - self.sin_theta * r.direction[2];
        direction[2] = self.sin_theta * r.direction[0] + self.cos_theta * r.direction[2];

        let rotated_r: Ray = Ray::new(origin, direction, r.time);

        if ! self.object.as_ref().unwrap().hit(&rotated_r, ray_t, rec) {
            return false;
        }

        let mut p: Vec3 = rec.p;
        p[0] = self.cos_theta * rec.p[0] + self.sin_theta * rec.p[2];
        p[2] = -self.sin_theta * rec.p[0] + self.cos_theta * rec.p[2];

        let mut normal: Vec3 = rec.normal;
        normal[0] = self.cos_theta * rec.normal[0] + self.sin_theta * rec.normal[2];
        normal[2] = -self.sin_theta * rec.normal[0] + self.cos_theta * rec.normal[2];

        rec.p = p;
        rec.normal = normal;

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}