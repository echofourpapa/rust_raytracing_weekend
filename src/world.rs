use std::cmp::Ordering;

use rand::Rng;

use crate::hittable::*;
use crate::interval::*;
use crate::ray::*;
use crate::material::*;
use crate::sphere::*;
use crate::vec3::*;
use crate::aabb::*;
use crate::bvh::*;

#[derive(Default, Clone)]
pub struct World {
    pub bvh_tree: Vec<Box<BVHNode>>,
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    pub materials: Vec<Box<dyn Material + Sync>>,
}

impl World {
    pub fn ray_color(&self, r: &Ray, depth: i32) -> Color {
        if depth <= 0 {
            return Color::zero();
        }
    
        let mut rec: HitRecord = HitRecord{..HitRecord::default()};
    
        if self.hit(r, Interval { min: 0.001, max: f64::INFINITY }, &mut rec) {
            let mut scattered: Ray = Ray{..Ray::default()};
            let mut attenuation: Color = Color::zero();
            let mat_idx: usize = rec.mat_idx;
            if self.materials[mat_idx].scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * self.ray_color(&scattered, depth-1); 
            }
            return Color::zero();
        }
        let unit_direction = normalize(r.direction);
        let t: f64 = 0.5 * (unit_direction.y() + 1.0);
        Color::one()*(1.0-t) + Color::new(0.5, 0.7, 1.0)*t
    }

    fn hit(&self, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // the BVH tree fills in reverse order, so start from the back
        let start: usize = self.bvh_tree.len()-1;
        self.hit_node(start, r, ray_t, rec)
    }

    fn hit_node(&self, idx:usize, r:&Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {

        let node: &BVHNode = self.bvh_tree[idx].as_ref();
        if !node.bbox.hit(r, ray_t) {
            return false;
        }

        if node.is_real {
            let hit_left: bool = self.objects[node.left].hit(r, ray_t, rec);
            let hit_right: bool = self.objects[node.right].hit( r, Interval { min: ray_t.min, max: if hit_left {rec.t} else {ray_t.max} }, rec);
            return hit_left || hit_right;
        }
        else {
            let hit_left: bool = self.hit_node(node.left, r, ray_t, rec);
            let hit_right: bool = self.hit_node(node.right, r, Interval { min: ray_t.min, max: if hit_left {rec.t} else {ray_t.max} }, rec);
            return hit_left || hit_right;
        }
    }

    pub fn create_material(self: &mut World, mat: Box<dyn Material + Sync>) -> usize {
        let mat_idx: usize = self.materials.len();
        self.materials.push(mat);
        mat_idx
    }

    pub fn build_bvh_tree(self: &mut World) {
        self.fill_bvh_nodes(0, self.objects.len());
    }

    pub fn fill_bvh_nodes(self: &mut World, start:usize, end:usize) -> usize {

        let object_span: usize = end - start;
        assert!(object_span != 0);

        let mut node: Box<BVHNode> = Box::new(BVHNode { ..BVHNode::default() });
        
        let axis: usize = rand::thread_rng().gen_range(0..=2) as usize;

        let comparator: fn(&AABB, &AABB) -> Ordering = match axis {
            0=>box_x_compare,
            1=>box_y_compare,
            _=>box_z_compare
        };
        
        if object_span == 1 {
            node.left = start;
            node.right = start;
            node.is_real = true;
            node.bbox = self.objects[start].bounding_box();
        } else if object_span == 2 {
            let mid: usize = start +1;
            if comparator(&self.objects[start].bounding_box(), &self.objects[start+1].bounding_box()) == Ordering::Less {
                node.left = start;
                node.right = mid;        
            } else {
                node.left = mid;
                node.right = start;        
            }
            node.is_real = true;
            node.bbox = self.objects[start].bounding_box() + self.objects[mid].bounding_box();
        } else {   
                     
            self.objects[start..end].sort_unstable_by(
                |a, b| 
                comparator(&a.bounding_box(), &b.bounding_box()) );
            let mid: usize = start + object_span/2;
            node.left = self.fill_bvh_nodes(start, mid);
            node.right = self.fill_bvh_nodes(mid, end);
            node.is_real = false;
            node.bbox = self.bvh_tree[node.left].bbox + self.bvh_tree[node.right].bbox;
        }

        let node_idx: usize = self.bvh_tree.len();
        self.bvh_tree.push(node);
        return node_idx;
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

pub fn random_world() -> World {
    let mut world: World = World{..World::default()};
    
    let ground_mat_idx: usize = world.create_material(Box::new(Lambertian{albedo: Color::new(0.5, 0.5,0.5)}));
    world.objects.push(Box::new(Sphere::new_static(Point3::new(0.0, -1000.0, -1.0), 1000.0, ground_mat_idx)));

    let left_mat_idx: usize = world.create_material(Box::new(Lambertian{albedo: Color::new(0.4, 0.2, 0.1)}));
    let center_mat_idx: usize = world.create_material(Box::new(Dielectric{ior:1.5}));
    let right_mat_idx: usize = world.create_material(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));

    world.objects.push(Box::new(Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, left_mat_idx)));
    world.objects.push(Box::new(Sphere::new_static(Point3::new( 0.0, 1.0, 0.0), 1.0, center_mat_idx)));
    world.objects.push(Box::new(Sphere::new_static(Point3::new( 4.0, 1.0, 0.0), 1.0, right_mat_idx)));

    let test_point: Vec3 = Point3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                rand::thread_rng().gen_range(0.0..=0.9) + a as f64,
                0.2,
                rand::thread_rng().gen_range(0.0..=0.9) + b as f64
            );

            let choose_mat: f64 = rand::thread_rng().gen_range(0.0..=1.0);

            if (center - test_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo:Color = Color::random() * Color::random();
                    let mat: Box<Lambertian> = Box::new(Lambertian{albedo: albedo});
                    let mat_idx = world.create_material(mat);

                    if choose_mat < 0.7 {
                        world.objects.push(Box::new(Sphere::new_static(center, 0.2, mat_idx)));
                    } else {
                        let center2 = center + Point3::new(
                            0.0,
                            rand::thread_rng().gen_range(0.0..=0.25) as f64,
                            0.0
                        );
                        world.objects.push(Box::new(Sphere::new_moving(center, center2, 0.2, mat_idx)));
                    }

                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo:Color = Color::random_range(0.5, 1.0);
                    let rough:f64 = rand::thread_rng().gen_range(0.0..=0.5);
                    let mat: Box<Metal> = Box::new(Metal::new(albedo, rough));
                    let mat_idx: usize = world.create_material(mat);
                    world.objects.push(Box::new(Sphere::new_static(center, 0.2, mat_idx)));
                }
                else {
                    // Glass
                    let mat: Box<Dielectric> = Box::new(Dielectric{ior:1.5});
                    let mat_idx: usize = world.create_material(mat);
                    world.objects.push(Box::new(Sphere::new_static(center, 0.2, mat_idx)));
                }
            }
        }
    }    

    world.build_bvh_tree();
    world
}