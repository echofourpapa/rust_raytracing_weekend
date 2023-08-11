use std::sync::Arc;

use crate::hittable_list::HittableList;
use crate::interval::*;
use crate::material::Material;
use crate::vec3::*;
use crate::ray::*;
use crate::hittable::*;
use crate::aabb::*;

#[derive(Clone)]
pub struct Quad {
    pub Q: Point3,
    pub u: Vec3,
    pub v: Vec3,
    pub mat: Option<Arc<dyn Material + Sync>>,
    pub bbox: AABB,
    pub normal: Vec3,
    pub D: f64,
    pub w: Vec3
}

impl Quad {
    pub fn new(Q:Point3, u:Vec3, v:Vec3, mat: &Arc<dyn Material + Sync>) -> Quad {
        let n: Vec3 = cross(&u,&v);
        let normal = normalize(n);
        Quad {
            Q:Q,
            u:u,
            v:v,
            mat: Some(mat.clone()),
            bbox: AABB::new(&Q, &(Q+ u + v)).pad(),
            normal: normal,
            D: dot(&normal, &Q),
            w: n / dot(&n, &n)
        }
    }
}

pub fn make_box(a: &Point3, b:&Point3, mat: &Arc<dyn Material + Sync>) -> HittableList {
    let mut sides: HittableList = HittableList{..Default::default()};

    let min: Vec3 = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max: Vec3 = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx: Vec3 = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy: Vec3 = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz: Vec3 = Vec3::new(0.0, 0.0, max.z() - min.z());

    sides.add_obj(Arc::new(Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dy, mat)));
    sides.add_obj(Arc::new(Quad::new(Point3::new(max.x(), min.y(), max.z()), -dz, dy, mat)));
    sides.add_obj(Arc::new(Quad::new(Point3::new(max.x(), min.y(), min.z()), -dx, dy, mat)));
    sides.add_obj(Arc::new(Quad::new(Point3::new(min.x(), min.y(), min.z()), dz, dy, mat)));
    sides.add_obj(Arc::new(Quad::new(Point3::new(min.x(), max.y(), max.z()), dx, -dz, mat)));
    sides.add_obj(Arc::new(Quad::new(Point3::new(min.x(), min.y(), max.z()), dx, dz, mat)));

    sides
}

pub fn is_interior(a: f64, b: f64, rec: &mut HitRecord) -> bool {
    if (a < 0.0) || (1.0 < a) || (b < 0.0) || (1.0 < b) {
        return false;
    }
    rec.u = a;
    rec.v = b;
    return true;
}

impl Hittable for Quad {
    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let denom: f64 = dot(&self.normal, &r.direction);
        if denom.abs() < f64::EPSILON {
            return false;
        }

        let t: f64 = (self.D - dot(&self.normal, &r.origin)) / denom;

        if !ray_t.contains(t) {
            return false;
        }

        let intersection: Vec3 = r.at(t);

        let planar_hitpt_vector: Vec3 = intersection - self.Q;
        let alpha: f64 = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta: f64 = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));
        if !is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);

        return true;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}