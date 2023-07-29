use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;

#[derive(Copy, Clone)]
pub struct Sphere {
    pub center: Point3,
    pub radius: f64,
    pub mat_idx: usize,
}

impl Hittable for Sphere {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool {

        let oc = r.origin - self.center;
        let a = r.direction.length_squared();
        let half_b = dot(&oc, &r.direction);
        let c = oc.length_squared() - self.radius*self.radius;
        let discriminant = half_b*half_b - a*c;
        if discriminant < 0.0 {
            return false;
        } 

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b -sqrtd ) / a;
            if root < t_min || t_max < root {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) /  self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_idx = self.mat_idx;
        
        return true;
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
        Box::new(self.clone())
    }

}