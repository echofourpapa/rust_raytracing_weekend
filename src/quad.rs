use crate::interval::*;
use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;
use crate::aabb::*;

#[derive(Copy, Clone)]
pub struct Quad {
    pub q: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub mat_idx: usize,
    pub bbox: AABB,
    pub normal: Vec3,
    pub d: f64,
    pub w: Vec3
}

impl Quad {
    pub fn new(q:Point3, u:Vec3, v:Vec3, mat_idx:usize) -> Quad {
        let mut n = cross(&u,&v);
        n.normalize();
        Quad {
            q:q,
            u:u,
            v:v,
            mat_idx:mat_idx,
            bbox: AABB::new(&q, &(q+ u + v)).pad(),
            normal: n,
            d: dot(&n, &q),
            w: n / dot(&n, &n)
        }
    }
}

pub fn is_interior(a: f64, b : f64) -> bool {
    if (a < 0.0 || 1.0 < a) || (b < 0.0 || 1.0 < b) {
        return false;
    }
    return true;
}

impl Hittable for Quad {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom: f64 = dot(&self.normal, &r.direction);
        if denom.abs() < f64::EPSILON {
            return false;
        }

        let t: f64 = (self.d - dot(&self.normal, &r.origin)) / denom;

        if !ray_t.contains(t) {
            return false;
        }

        let intersection: Vec3 = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha: f64 = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta: f64 = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !is_interior(alpha, beta) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat_idx = self.mat_idx;
        rec.set_face_normal(r, &self.normal);

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }

    fn clone_dyn(&self) -> Box<dyn Hittable + Sync> {
        Box::new(self.clone())
    }
}