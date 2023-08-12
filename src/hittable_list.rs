use std::sync::Arc;

use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::aabb::*;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Sync>>,
    pub bbox: AABB
}

impl HittableList {
    #[allow(dead_code)]
    pub fn new(obj: Arc<dyn Hittable + Sync>) -> HittableList {
        let mut h: HittableList = HittableList::default();
        h.add_obj(obj);
        h
    }

    pub fn add_obj(self: &mut HittableList, obj: Arc<dyn Hittable + Sync>) {
        self.bbox += obj.bounding_box();
        self.objects.push(obj);
    }
}

impl Hittable for HittableList {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let mut closest_so_far: f64 = ray_t.max;
        let mut hit_anything: bool = false;
        
        for object in self.objects.iter() {
            if object.hit(r, Interval{min:ray_t.min, max:closest_so_far}, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
    
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}