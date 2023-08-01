use crate::vec3::*;
use crate::ray::*;

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
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool;
    fn clone_dyn(&self) -> Box<dyn Hittable + Sync>;
}

impl Clone for Box<dyn Hittable + Sync> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}