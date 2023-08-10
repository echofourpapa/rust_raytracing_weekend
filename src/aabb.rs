use std::ops;

use crate::interval::*;
use crate::vec3::*;
use crate::ray::*;

#[derive(Copy, Clone, Default)]
pub struct AABB {
    x: Interval,
    y: Interval,
    z: Interval
}

impl AABB {

    pub fn new(a: &Point3, b: &Point3) -> AABB {
        AABB {
            x: Interval { min: a.x().min(b.x()), max: a.x().max(b.x()) },
            y: Interval { min: a.y().min(b.y()), max: a.y().max(b.y()) },
            z: Interval { min: a.z().min(b.z()), max: a.z().max(b.z()) }
        }
    }

    pub fn axis(&self, n:usize)-> &Interval {
        assert!(n <=2);
        match n {
            1=> &self.y,
            2=> &self.z,
            _=> &self.x
        }
    }

    pub fn hit(&self, r:&Ray, ray_t: Interval) -> bool {
        for a in 0..3 {
            let inv_d: f64 = 1.0 / r.direction[a];
            let orig: f64 = r.origin[a];
            let axis: &Interval = self.axis(a);
            let mut t0: f64 = (axis.min - orig) * inv_d;
            let mut t1: f64 = (axis.max - orig) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min_t: f64 = if t0 > ray_t.min {t0} else {ray_t.min};
            let t_max_t: f64 = if t1 < ray_t.max {t1} else {ray_t.max};
            if t_max_t <= t_min_t {
                return false;
            }
        }
        return true;
    }

    pub fn pad(&self) -> AABB {
        let delta: f64 = 0.0001;
        AABB {
            x: if self.x.size() >= delta {self.x} else {self.x.expand(delta)},
            y: if self.y.size() >= delta {self.y} else {self.y.expand(delta)},
            z: if self.z.size() >= delta {self.z} else {self.z.expand(delta)}
        }
    }
}

impl ops::Add<AABB> for AABB {
    type Output = AABB;
    fn add(self, other: AABB) -> AABB {
        AABB {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        }
    }
}

impl ops::AddAssign<AABB> for AABB {
    fn add_assign(&mut self, other: AABB) {
        *self = AABB {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z
        };
    }
}