use std::sync::Arc;

use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::material::*;
use crate::aabb::*;
use crate::vec3::Color;
use crate::vec3::normalize;

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

    pub fn ray_color(&self, r: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::zero();
        }
    
        let mut rec: HitRecord = HitRecord{..HitRecord::default()};
    
        if self.hit(r, Interval { min: 0.001, max: f64::INFINITY }, &mut rec) {
            let mut scattered: Ray = Ray{..Ray::default()};
            let mut attenuation: Color = Color::zero();
            let mat: &Arc<dyn Material + Sync> = rec.mat.as_ref().unwrap();
            if mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                // Uncomment to debug UVs
                // attenuation.set_x(rec.u);
                // attenuation.set_y(rec.v);
                // attenuation.set_z(0.0);
                return attenuation * self.ray_color(&scattered, depth-1); 
            }
            return Color::zero();
        }
        let unit_direction = normalize(r.direction);
        let t: f64 = 0.5 * (unit_direction.y() + 1.0);
        Color::one()*(1.0-t) + Color::new(0.5, 0.7, 1.0)*t
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

    // fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
    //     Box::new(self.clone())
    // }
}