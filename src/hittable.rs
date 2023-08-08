use crate::interval::*;
use crate::vec3::*;
use crate::ray::*;
use crate::aabb::*;

#[derive(Copy, Clone, Default)]
pub struct HitRecord {
    pub p:Point3,
    pub normal: Vec3,
    pub mat_idx: usize,
    pub t: f64,
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
    fn clone_dyn(&self) -> Box<dyn Hittable + Sync>;
}

impl Clone for Box<dyn Hittable + Sync> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}