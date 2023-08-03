use rand::Rng;

use crate::hittable::*;
use crate::ray::*;
use crate::material::*;
use crate::sphere::*;
use crate::vec3::*;

#[derive(Default, Clone)]
pub struct HittableList {
    pub objects: Vec<Box<dyn Hittable + Sync>>,
    pub materials: Vec<Box<dyn Material + Sync>>,
}

impl HittableList {
    pub fn hit(&self, r:&Ray, t_min:f64, t_max:f64, rec: &mut HitRecord) -> bool {
        let mut closest_so_far = t_max;
        let mut hit_anything = false;
        
        for object in self.objects.iter() {
            if object.hit(r, t_min, closest_so_far, rec) {
                hit_anything = true;
                closest_so_far = rec.t;
            }
        }
        hit_anything
    }

    pub fn create_material(self: &mut HittableList, mat: Box<dyn Material + Sync>) -> usize {
        let mat_idx = self.materials.len();
        self.materials.push(mat);
        mat_idx
    }
}

pub fn random_world() -> HittableList {
    let mut world = HittableList{..HittableList::default()};
    
    let ground_mat_idx = world.create_material(Box::new(Lambertian{albedo: Color::new(0.5, 0.5,0.5)}));
    world.objects.push(Box::new(Sphere{ center: Point3::new(0.0, -1000.0, -1.0), radius: 1000.0, mat_idx:ground_mat_idx}));

    let test_point = Point3::new(4.0, 0.2, 0.0);

    for a in -11..11 {
        for b in -11..11 {
            let center = Point3::new(
                rand::thread_rng().gen_range(0.0..=0.9) + a as f64,
                0.2,
                rand::thread_rng().gen_range(0.0..=0.9) + b as f64
            );

            let choose_mat = rand::thread_rng().gen_range(0.0..=1.0);

            if (center - test_point).length() > 0.9 {
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo:Color = Color::random() * Color::random();
                    let mat: Box<Lambertian> = Box::new(Lambertian{albedo: albedo});
                    let mat_idx = world.create_material(mat);

                    let center2 = center + Point3::new(
                        0.0,
                        rand::thread_rng().gen_range(0.0..=0.5) as f64,
                        0.0
                    );

                    world.objects.push(Box::new(MovingSphere{ 
                        start_pos: center, 
                        end_pos: center2, 
                        radius: 0.2, 
                        mat_idx:mat_idx,
                        time: 1.0
                    }));

                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo:Color = Color::random_range(0.5, 1.0);
                    let rough:f64 = rand::thread_rng().gen_range(0.0..=0.5);
                    let mat: Box<Metal> = Box::new(Metal::new(albedo, rough));
                    let mat_idx = world.create_material(mat);
                    world.objects.push(Box::new(Sphere{ center: center, radius: 0.2, mat_idx:mat_idx}));
                }
                else {
                    // Glass
                    let mat: Box<Dielectric> = Box::new(Dielectric{ior:1.5});
                    let mat_idx = world.create_material(mat);
                    world.objects.push(Box::new(Sphere{ center: center, radius: 0.2, mat_idx:mat_idx}));
                }
            }
        }
    }

    

    let left_mat_idx = world.create_material(Box::new(Lambertian{albedo: Color::new(0.4, 0.2, 0.1)}));
    let center_mat_idx = world.create_material(Box::new(Dielectric{ior:1.5}));
    let right_mat_idx = world.create_material(Box::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0)));

    world.objects.push(Box::new(Sphere{ center: Point3::new(-4.0, 1.0, 0.0), radius: 1.0, mat_idx:left_mat_idx}));
    world.objects.push(Box::new(Sphere{ center: Point3::new( 0.0, 1.0, 0.0), radius: 1.0, mat_idx:center_mat_idx}));
    world.objects.push(Box::new(Sphere{ center: Point3::new( 4.0, 1.0, 0.0), radius: 1.0, mat_idx:right_mat_idx}));
    
    world
}