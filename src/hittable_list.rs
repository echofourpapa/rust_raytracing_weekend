use crate::hittable::*;
use crate::ray::*;
use crate::material::*;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    pub materials: Vec<Box<dyn Material + Sync>>,
}

impl HittableList {
    pub fn hit(&self, r:&Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool {
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        
        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
                //rec = &mut temp_rec;
            }
        }
        hit_anything
    }
}