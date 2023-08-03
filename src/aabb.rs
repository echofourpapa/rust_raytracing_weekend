use crate::vec3::*;
use crate::ray::*;

#[derive(Copy, Clone, Default)]
pub struct AABB {
    pub min: Point3,
    pub max: Point3,
}

impl AABB {
    pub fn hit(&self, r:&Ray, t_min:f64, t_max:f64) -> bool {
        for a in 0..3 {
            let inv_d = 1.0 / r.direction[a];
            let mut t0 = (self.min[a] - r.origin[a]) * inv_d;
            let mut t1 = (self.max[a] - r.origin[a]) * inv_d;
            if inv_d < 0.0 {
                std::mem::swap(&mut t0, &mut t1);
            }
            let t_min_t = if t0 > t_min {t0} else {t_min};
            let t_max_t = if t1 < t_max {t1} else {t_max};
            if t_max_t <= t_min_t {
                return false;
            }
        }
        return true;
    }
}

pub fn surrounding_box(box0: &AABB, box1: &AABB) -> AABB {
    let min = Vec3::new(
        box0.min.x().min(box1.min.x()),
        box0.min.y().min(box1.min.y()),
        box0.min.z().min(box1.min.z())
    );

    let max = Vec3::new(
        box0.max.x().min(box1.max.x()),
        box0.max.y().min(box1.max.y()),
        box0.max.z().min(box1.max.z())
    );

    AABB {
        min:min,
        max:max
    }
}