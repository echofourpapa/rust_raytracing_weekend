use std::sync::Arc;

use rand::Rng;

use crate::hittable::*;
use crate::hittable_list::HittableList;
use crate::quad::*;
use crate::material::*;
use crate::sphere::*;
use crate::vec3::*;
use crate::bvh::*;

pub fn quads() -> HittableList {
    let mut l_world: HittableList = HittableList{..HittableList::default()};

    let left_red     : Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(1.0, 0.2, 0.2)});
    let back_green   : Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.2, 1.0, 0.2)});
    let right_blue   : Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.2, 0.2, 1.0)});
    let upper_orange : Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(1.2, 0.5, 0.5)});
    let lower_teal   : Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.2, 0.8, 0.8)});

    l_world.add_obj(Arc::new(Quad::new(Point3::new(-3.0, -2.0, 5.0), Point3::new(0.0, 0.0,-4.0), Point3::new(0.0, 4.0, 0.0), &left_red)));
    l_world.add_obj(Arc::new(Quad::new(Point3::new(-2.0, -2.0, 0.0), Point3::new(4.0, 0.0, 0.0), Point3::new(0.0, 4.0, 0.0), &back_green)));
    l_world.add_obj(Arc::new(Quad::new(Point3::new(3.0, -2.0, 1.0), Point3::new(0.0, 0.0, 4.0), Point3::new(0.0, 4.0, 0.0), &right_blue)));
    l_world.add_obj(Arc::new(Quad::new(Point3::new(-2.0, 3.0, 1.0), Point3::new(4.0, 0.0, 0.0), Point3::new(0.0, 0.0, 4.0), &upper_orange)));
    l_world.add_obj(Arc::new(Quad::new(Point3::new(-2.0, -3.0, 5.0), Point3::new(4.0, 0.0,0.0), Point3::new(0.0, 0.0,-4.0), &lower_teal)));

    let mut world: HittableList = HittableList{..HittableList::default()};
    world.add_obj(Arc::new(BVHNode::new_list(&l_world)));
    world
}

pub fn cornell_box() -> HittableList {
    let mut l_world: HittableList = HittableList{..HittableList::default()};

    let red  : Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.65, 0.05,0.05)});
    let white: Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.73, 0.73,0.73)});
    let green: Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.12, 0.45,0.15)});
    let light: Arc<dyn Material + Sync> = Arc::new(Emiter{ emission: Color::new(15.0, 15.0, 15.0) });

    l_world.add_obj(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0), 
        &green
    )));
    l_world.add_obj(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0),
        Point3::new(0.0, 0.0, 555.0), 
        &red
    )));
    l_world.add_obj(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Point3::new(-130.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -105.0), 
        &light
    )));
    l_world.add_obj(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, 555.0), 
        &white
    )));
    l_world.add_obj(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Point3::new(-555.0, 0.0, 0.0),
        Point3::new(0.0, 0.0, -555.0), 
        &white
    )));
    l_world.add_obj(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Point3::new(555.0, 0.0, 0.0),
        Point3::new(0.0, 555.0, 0.0), 
        &white
    )));

    let mut box1: Arc<dyn Hittable + Sync> = Arc::new(make_cube(&white));
    box1 = Arc::new(Scale::new(box1, Vec3::new(165.0, 330.0, 165.0)));
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0,0.0,295.0)));
    
    l_world.add_obj(box1);

    let mut box2: Arc<dyn Hittable + Sync> = Arc::new(make_cube(&white));
    box2 = Arc::new(Scale::new(box2, Vec3::new(165.0, 165.0, 165.0)));
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0,0.0,65.0)));
    l_world.add_obj(box2);

    let mut world: HittableList = HittableList{..HittableList::default()};
    world.add_obj(Arc::new(BVHNode::new_list(&l_world)));
    world
}

pub fn random_world() -> HittableList {
    let mut l_world: HittableList = HittableList{..HittableList::default()};

    let ground_mat: Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.5, 0.5,0.5)});

    l_world.add_obj(Arc::new(Sphere::new_static(Point3::new(0.0, -1000.0, -1.0), 1000.0, &ground_mat)));

    let left_mat: Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: Color::new(0.4, 0.2, 0.1)});
    let center_mat: Arc<dyn Material + Sync> = Arc::new(Dielectric{ior:1.5});
    let right_mat: Arc<dyn Material + Sync> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));

    l_world.add_obj(Arc::new(Sphere::new_static(Point3::new(-4.0, 1.0, 0.0), 1.0, &left_mat)));
    l_world.add_obj(Arc::new(Sphere::new_static(Point3::new( 0.0, 1.0, 0.0), 1.0, &center_mat)));
    l_world.add_obj(Arc::new(Sphere::new_static(Point3::new( 4.0, 1.0, 0.0), 1.0, &right_mat)));

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
                    let mat: Arc<dyn Material + Sync> = Arc::new(Lambertian{albedo: albedo});
                    if choose_mat < 0.7 {
                        l_world.add_obj(Arc::new(Sphere::new_static(center, 0.2, &mat)));
                    } else {
                        let center2 = center + Point3::new(
                            0.0,
                            rand::thread_rng().gen_range(0.0..=0.25) as f64,
                            0.0
                        );
                        l_world.add_obj(Arc::new(Sphere::new_moving(center, center2, 0.2, &mat)));
                    }

                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo:Color = Color::random_range(0.5, 1.0);
                    let rough:f64 = rand::thread_rng().gen_range(0.0..=0.5);
                    let mat: Arc<dyn Material + Sync> = Arc::new(Metal::new(albedo, rough));
                    l_world.add_obj(Arc::new(Sphere::new_static(center, 0.2, &mat)));
                }
                else {
                    // Glass
                    let mat: Arc<dyn Material + Sync> = Arc::new(Dielectric{ior:1.5});
                    l_world.add_obj(Arc::new(Sphere::new_static(center, 0.2, &mat)));
                }
            }
        }
    }    

    let mut world: HittableList = HittableList{..HittableList::default()};
    world.add_obj(Arc::new(BVHNode::new_list(&l_world)));
    world
}