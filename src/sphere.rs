use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;
use crate::aabb::*;

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

    fn bounding_box(&self, _delta: f64, out_box: &mut AABB) -> bool {
        out_box.min = self.center - Vec3::new(self.radius, self.radius, self.radius);
        out_box.max = self.center + Vec3::new(self.radius, self.radius, self.radius);
        true
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
        Box::new(self.clone())
    }
}

#[derive(Copy, Clone)]
pub struct MovingSphere {
    pub start_pos: Point3,
    pub end_pos: Point3,
    pub radius: f64,
    pub mat_idx: usize,
    pub time: f64
}

impl MovingSphere {
    pub fn center(&self, time:f64) -> Point3 {
        // self.start_pos + (time/self.time) * self.end_pos-self.start_pos
        lerp(&self.start_pos, &self.end_pos, time)
    }
}

impl Hittable for MovingSphere {
    fn hit(&self, r:&Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool {

        let oc = r.origin - self.center(r.time);
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
        let outward_normal = (rec.p - self.center(r.time)) /  self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat_idx = self.mat_idx;

        return true;
    }

    fn bounding_box(&self, delta: f64, out_box: &mut AABB) -> bool {
        let box0 = AABB{ 
            min: self.center(0.0) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(0.0) + Vec3::new(self.radius, self.radius, self.radius)
        };

        let box1 = AABB{ 
            min: self.center(delta) - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center(delta) + Vec3::new(self.radius, self.radius, self.radius)
        };
        let big_box = surrounding_box(&box0, &box1);
        out_box.min = big_box.min;
        out_box.max = big_box.max;
        true
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
        Box::new(self.clone())
    }

}