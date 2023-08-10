use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::material::*;
use crate::aabb::*;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    pub materials: Vec<Box<dyn Material + Sync>>,
    pub bbox: AABB
}

impl HittableList {
    pub fn create_material(self: &mut HittableList, mat: Box<dyn Material + Sync>) -> usize {
        let mat_idx = self.materials.len();
        self.materials.push(mat);
        mat_idx
    }

    pub fn add_obj(self: &mut HittableList, obj: Box<dyn Hittable + Sync>) {
        self.bbox += obj.bounding_box();
        self.objects.push(obj);
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
    
    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
        Box::new(self.clone())
    }

    fn get_mat(&self) -> usize {
        todo!()
    }

    fn set_mat(&mut self, mat_idx: usize) {
        todo!()
    }
}