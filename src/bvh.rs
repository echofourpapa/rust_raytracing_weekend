use std::{sync::Arc, cmp::Ordering};

use rand::Rng;

use crate::{aabb::*, hittable::Hittable, hittable_list::HittableList, interval::Interval};

#[derive(Clone, Default)]
pub struct BVHNode {
    pub left: Option<Arc<dyn Hittable + Sync>>,
    pub right: Option<Arc<dyn Hittable + Sync>>,
    pub bbox: AABB
}

impl BVHNode {

    pub fn new_list(hlist: &HittableList) -> BVHNode {
        BVHNode::new(&hlist.objects, 0, hlist.objects.len())
    }

    pub fn new(src_ojects: &Vec<Arc<dyn Hittable + Sync>>, start:usize, end:usize) -> BVHNode {

        let mut objects:Vec<Arc<dyn Hittable + Sync>> = src_ojects.clone();

        let object_span: usize = end - start;
        assert!(object_span != 0);        
        
        let axis: usize = rand::thread_rng().gen_range(0..=2) as usize;
        let comparator: fn(&AABB, &AABB) -> Ordering = match axis {
            0=>box_x_compare,
            1=>box_y_compare,
            _=>box_z_compare
        };

        let mut node = BVHNode { ..BVHNode::default() };
        
        if object_span == 1 {
            node.left = Some(objects[start].clone());
            node.right = Some(objects[start].clone());

        } else if object_span == 2 {
            let mid: usize = start +1;
            if comparator(&objects[start].bounding_box(), &objects[start+1].bounding_box()) == Ordering::Less {
                node.left = Some(objects[start].clone());
                node.right = Some(objects[mid].clone());        
            } else {
                node.left = Some(objects[mid].clone());
                node.right = Some(objects[start].clone());        
            }
            
        } else {   
            objects[start..end].sort_by(
                |a, b| 
                comparator(&a.bounding_box(), &b.bounding_box()) );
            let mid: usize = start + object_span/2;
            node.left = Some(Arc::new(BVHNode::new(&objects, start, mid)));
            node.right = Some(Arc::new(BVHNode::new(&objects, mid, end)));
        }

        node.build_bounding_box();
        node
    }

    fn build_bounding_box(&mut self) {
        self.bbox = self.left.as_ref().unwrap().bounding_box() + self.right.as_ref().unwrap().bounding_box();
    }

}

fn box_compare(a: &AABB, b: &AABB, axis: usize) -> Ordering {
    return if a.axis(axis).min < b.axis(axis).min {Ordering::Less} else {Ordering::Greater}
}

fn box_x_compare(a: &AABB, b: &AABB) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &AABB, b: &AABB) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &AABB, b: &AABB) -> Ordering {
    box_compare(a, b, 2)
}

impl Hittable for BVHNode {
    fn hit(&self, r:&crate::ray::Ray, ray_t: crate::interval::Interval, rec: &mut crate::hittable::HitRecord) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left: bool = self.left.as_ref().unwrap().hit(r, ray_t, rec);

        let t_max: f64 = if hit_left {rec.t} else {ray_t.max};
        let hit_right: bool = self.right.as_ref().unwrap().hit(r, Interval { min: ray_t.min, max: t_max }, rec);

        return hit_left || hit_right;
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}