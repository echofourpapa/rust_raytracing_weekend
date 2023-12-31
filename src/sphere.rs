use std::f64::consts::PI;
use std::sync::Arc;

use crate::interval::*;
use crate::material::Material;
use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;
use crate::aabb::*;

#[derive(Clone)]
pub struct Sphere {
    pub center: Point3,
    pub vector: Point3,
    pub radius: f64,
    pub mat: Option<Arc<dyn Material + Sync>>,
    pub is_moving: bool,
    pub bbox: AABB
}

pub fn get_sphere_uvw(p:&Point3) -> Vec3 {
    let theta: f64 = (-p.y()).acos();
    let phi:f64 = (-p.z()).atan2(p.x()) + PI;

    let u: f64 = phi / (2.0*PI);
    let v: f64 = theta / PI;
    Vec3::new(u, v, 0.0)
}

impl Sphere {
    pub fn new_static(center: Point3, radius: f64, mat: &Arc<dyn Material + Sync>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        Sphere { 
            center: center, 
            vector: Vec3::zero(),
            radius: radius, 
            mat: Some(mat.clone()), 
            is_moving: false,
            bbox: AABB::new(&(center-rvec), &(center+rvec) )
        }
    }

    pub fn new_moving(center: Point3, end: Vec3, radius: f64,  mat: &Arc<dyn Material + Sync>) -> Sphere {
        let rvec = Vec3::new(radius, radius, radius);
        let bbox1 = AABB::new(&(center-rvec), &(center+rvec) );
        let bbox2 = AABB::new(&(end-rvec), &(end+rvec) );
        Sphere { 
            center: center, 
            vector: end - center,
            radius: radius, 
            mat: Some(mat.clone()), 
            is_moving: true,
            bbox: bbox1 + bbox2 
        }
    }

    pub fn center(&self, time:f64) -> Point3 {
        // self.start_pos + (time/self.time) * self.end_pos-self.start_pos
        // lerp(&self.center, &(self.center + self.vector), time)
        if self.is_moving {
            self.center + self.vector*time
        }
        else {
            self.center
        }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {

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
        if !ray_t.surrounds(root) {
            root = (-half_b -sqrtd ) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center(r.time)) /  self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.uvw = get_sphere_uvw(&outward_normal);
        rec.mat = self.mat.clone();

        return true;
    }
    
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}