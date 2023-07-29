use std::{io::{stdout, Write}, sync::{Arc, Mutex}, time::Instant};
use hittable::HitRecord;
use rand::Rng;
use std::thread;

use ray::*;
use vec3::*;
use sphere::*;
use hittable_list::*;
use camera::*;
use material::*;
use common::*;

mod tga;
mod vec3;
mod ray;
mod sphere;
mod hittable;
mod hittable_list;
mod camera;
mod material;
mod common;


fn write_color(buffer: &mut Vec<u8>, color:&Color, spp: u32, pos:usize) {

    let scale = 1.0 / (spp as f64);
    let scaled_color = *color as Vec3 * scale;

    let r = scaled_color.x.sqrt();
    let g = scaled_color.y.sqrt();
    let b = scaled_color.z.sqrt();

    buffer[pos]   =  (255.0 * Saturate(b)) as u8;
    buffer[pos+1] =  (255.0 * Saturate(g)) as u8;
    buffer[pos+2] =  (255.0 * Saturate(r)) as u8;
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {

    if depth <= 0 {
        return Color{x:0.0, y:0.0, z:0.0};
    }

    let mut rec = HitRecord{..HitRecord::default()};

    if  world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray{..Ray::default()};
        let mut attenuation: Color = Color {..Color::default()};
        let mat_idx: usize = rec.mat_idx;
        if world.materials[mat_idx].scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1); 
        }
        return Color{x:0.0, y:0.0, z:0.0};
    }
    let unit_direction = normalize(r.direction);
    let t = 0.5 * (unit_direction.y + 1.0);
    Color{x:1.0, y:1.0, z:1.0}*(1.0-t) + Color{x:0.5, y: 0.7, z: 1.0}*t
}

fn main() -> Result<(), std::io::Error> {

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width: i32 = 1920;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 256;
    let max_depth = 50;

    let start = Instant::now();

    // World
    let mut world = HittableList{ ..HittableList::default()};

    let ground_mat_idx = world.materials.len();
    world.materials.push(Box::new(Lambertian{albedo: Color{x:0.8, y: 0.8, z:0.0}}));
    let center_mat_idx = world.materials.len();
    world.materials.push(Box::new(Lambertian{albedo: Color{x:0.7, y: 0.3, z:0.3}}));
    let left_mat_idx = world.materials.len();
    world.materials.push(Box::new(Metal::new( Color{x:0.8, y: 0.8, z:0.8}, 0.3)));
    let right_mat_idx = world.materials.len();
    world.materials.push(Box::new(Metal::new(Color{x:0.8, y: 0.6, z:0.2}, 1.0)));

    world.objects.push(Box::new(Sphere{ center: Point3{x:0.0, y: -100.5, z:-1.0}, radius: 100.0, mat_idx:ground_mat_idx}));
    world.objects.push(Box::new(Sphere{ center: Point3{x:0.0, y: 0.0, z:-1.0}, radius: 0.5, mat_idx:center_mat_idx}));
    world.objects.push(Box::new(Sphere{ center: Point3{x:-1.0, y: 0.0, z:-1.0}, radius: 0.5, mat_idx:left_mat_idx}));
    world.objects.push(Box::new(Sphere{ center: Point3{x:1.0, y: 0.0, z:-1.0}, radius: 0.5, mat_idx:right_mat_idx}));
    

    // Camera
    let cam = Camera::new();

    let mut children_threads = vec![];

    println!("Image size: {}x{}", image_width, image_height);
    let size = image_width * image_width * 3;
    let image_buffer = Arc::new(Mutex::new(vec![0; size as usize]));

    let world_arc = Arc::new(world);
    for y in 0..image_height {
        print!("\r Rendering line {} of {}", y+1, image_height);
        children_threads.push(thread::spawn( {
            let clone = Arc::clone(&image_buffer);
            let world_clone = world_arc.clone();
            move || {
                for x in 0..image_width {
                    let mut pixel_color = Color{..Color::default()};
                    for _s in 0..samples_per_pixel {
                        let a: f64 = rand::thread_rng().gen();
                        let b: f64 = rand::thread_rng().gen();
                        let u: f64 = (x as f64 + a) / ((image_width-1) as f64);
                        let v: f64 = (y as f64 + b) / ((image_height-1) as f64);
                        let r = cam.get_ray(u, v);
                        pixel_color += ray_color(&r, &world_clone, max_depth);
                    }
                    let pos: i32 = (x + y * image_width) * 3;
                    let mut v = clone.lock().unwrap();
                    write_color(&mut v, &pixel_color, samples_per_pixel, pos as usize);
                }
            }
        }));
        stdout().flush().unwrap();
    }
    print!("\nWaiting for threads to finish.\n");
    let mut t_count = 0;

    for child in children_threads {
        // Wait for the thread to finish. Returns a result.
        let _ = child.join().unwrap();
        t_count += 1;
        print!("\r Finishing line {} of {}", t_count, image_height);
    }

    print!("\n");
    let file_path: &str = "D:/code/rust/raytracing_weekend/test_image.tga";
    println!("Saving to: {}", file_path);
    tga::write_tga_file(image_width, image_height, &*image_buffer.lock().unwrap(), file_path)?;
    println!("Done! Completed in {:?}", start.elapsed());
    Ok(())
}
