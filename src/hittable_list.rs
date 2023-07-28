use crate::hittable::*;
use crate::ray::*;
use crate::sphere::*;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Sphere>,
}

// impl Clone for Box<dyn Hittable> {
//     fn clone(&self) -> Self {
//         self.clone_dyn()
//     }
// }

impl HittableList {
    pub fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> HitRecord {
        let mut closest_so_far = t_max;
        let mut temp_rec = HitRecord { ..HitRecord::default() };

        for object in self.objects.iter() {
            let r: HitRecord = object.hit(r, t_min, closest_so_far);
            if r.hit {
                closest_so_far = temp_rec.t;
                temp_rec = r
            }
        }
        temp_rec
    }
}