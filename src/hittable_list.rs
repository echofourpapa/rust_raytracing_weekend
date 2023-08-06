use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::material::*;
use crate::aabb::*;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    pub materials: Vec<Box<dyn Material + Sync>>,
}

impl HittableList {
    pub fn create_material(self: &mut HittableList, mat: Box<dyn Material + Sync>) -> usize {
        let mat_idx = self.materials.len();
        self.materials.push(mat);
        mat_idx
    }
}

impl Hittable for HittableList {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut closest_so_far = ray_t.max;
        let mut hit_anything = false;
        
        for object in self.objects.iter() {
            if object.hit(r, Interval{min:ray_t.min, max:closest_so_far}, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }
    
    fn bounding_box(&self, delta: f64, out_box: &mut AABB) -> bool {
        if self.objects.is_empty(){
            return false;
        }

        let mut temp_box =  AABB{..AABB::default()};
        let mut first_box = true;

        for object in self.objects.iter() {
            if !object.bounding_box(delta, &mut temp_box) {
                return false;
            } else {
                if first_box {
                    out_box.min = temp_box.min;
                    out_box.max = temp_box.max;
                } else {
                     let big_box =  surrounding_box(&out_box,&temp_box);
                     out_box.min = big_box.min;
                     out_box.max = big_box.max;
                }
                first_box = false;
            }
        }
 
        return true;
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
        Box::new(self.clone())
    }
}