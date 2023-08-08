use crate::aabb::*;

#[derive(Copy, Clone, Default)]
pub struct BVHNode {
    pub left: usize,
    pub right: usize,
    pub is_real: bool,
    pub bbox: AABB
}