use crate::hittable::*;
use crate::aabb::*;
use crate::interval::*;

#[derive(Copy, Clone, Default)]
pub struct BVHNode {
    pub left: usize,
    pub right: usize,
    pub bbox: AABB
}

// impl BVHNode {
//     pub fn new(list: &HittableList, delta:f64) -> BVHNode {

//     }

//     pub fn new_build()
// }

impl Hittable for BVHNode {
    fn hit(&self, r:&crate::ray::Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        todo!()
    }

    fn bounding_box(&self, delta: f64, out_box: &mut AABB) -> bool {
        todo!()
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
        todo!()
    }
}