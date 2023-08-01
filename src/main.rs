use std::{io::{stdout, Write}, sync::{Arc, Mutex}, time::{Instant, Duration}, env};
use hittable::HitRecord;
use rand::Rng;
use std::thread;
use threadpool::ThreadPool;

use ray::*;
use vec3::*;
use hittable_list::*;
use camera::*;
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

    let r = scaled_color.x().sqrt();
    let g = scaled_color.y().sqrt();
    let b = scaled_color.z().sqrt();

    buffer[pos]   =  (255.0 * saturate(b)) as u8;
    buffer[pos+1] =  (255.0 * saturate(g)) as u8;
    buffer[pos+2] =  (255.0 * saturate(r)) as u8;
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {

    if depth <= 0 {
        return Color::zero();
    }

    let mut rec = HitRecord{..HitRecord::default()};

    if  world.hit(r, 0.001, f64::INFINITY, &mut rec) {
        let mut scattered: Ray = Ray{..Ray::default()};
        let mut attenuation: Color = Color::zero();
        let mat_idx: usize = rec.mat_idx;
        if world.materials[mat_idx].scatter(r, &rec, &mut attenuation, &mut scattered) {
            return attenuation * ray_color(&scattered, world, depth-1); 
        }
        return Color::zero();
    }
    let unit_direction = normalize(r.direction);
    let t = 0.5 * (unit_direction.y() + 1.0);
    Color::one()*(1.0-t) + Color::new(0.5, 0.7, 1.0)*t
}

fn main() -> Result<(), std::io::Error> {

    // Image
    let aspect_ratio = 16.0/9.0;
    let image_width: i32 = 1920;
    let image_height: i32 = (image_width as f64 / aspect_ratio) as i32;
    let samples_per_pixel = 512;
    let max_depth = 50;

    

    // World
    let world = random_world();
   
    // Camera
    let cam_origin = Point3::new(13.0, 2.0, 3.0);
    let cam_target = Point3::new(0.0, 0.0, 0.0);

    let cam = Camera::new(
        cam_origin,
        cam_target, 
        Vec3::up(), 
        20.0, 
        aspect_ratio,
        0.1,
        10.0
    );

    // let mut children_threads = vec![];

    let max_threads = thread::available_parallelism().unwrap().get() - 1;
    let pool = ThreadPool::new(max_threads);

    println!("Image size: {}x{}", image_width, image_height);
    let size = image_width * image_width * 3;
    let image_buffer = Arc::new(Mutex::new(vec![0; size as usize]));

    let world_arc = Arc::new(world);

    let line_step = image_width / max_threads as i32;

    // Start timer
    let start = Instant::now();
    let st = start.elapsed().as_secs_f64();

    for y in 0..image_height {
        for i in 0..line_step {
            pool.execute( {
                let clone = Arc::clone(&image_buffer);
                let world_clone = world_arc.clone();
                move || {
                    let scanline_start = (i * line_step).min(image_width);
                    let scanline_end = (scanline_start + line_step).min(image_width);
                    for x in scanline_start..scanline_end {
                        let mut pixel_color = Color::zero();
                        for _s in 0..samples_per_pixel {
                            let a: f64 = rand::thread_rng().gen_range(0.0..=1.0);
                            let b: f64 = rand::thread_rng().gen_range(0.0..=1.0);
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
            });   
        }
    }

    let total_possible_threads = image_height * line_step;

    while pool.queued_count() > 0 && pool.active_count() > 0 {
        thread::sleep(Duration::from_millis(100));
        let total:usize = pool.active_count() + pool.queued_count();
        let finished: i32 = total_possible_threads - total as i32;
        let prog: f64 = finished as f64 / total_possible_threads as f64;
        let t = start.elapsed().as_secs_f64();
        let estimate = (t/finished as f64) * total_possible_threads as f64;
        print!("\r{} Active, {} Queued, {:.2}% Complete, Running time: {:.2}s, Time Remaining {}",
            pool.active_count(), 
            pool.queued_count(),
            prog * 100.0,
            seconds_to_hhmmss(t),
            seconds_to_hhmmss(estimate - t)
        );
        stdout().flush().unwrap();

    }
    
    pool.join();

    print!("\n");
    let mut file_path = env::current_dir().unwrap();
    file_path.set_file_name("test_image.tga");
    println!("Saving to: {}", file_path.display());
    tga::write_tga_file(image_width, image_height, &*image_buffer.lock().unwrap(), &file_path)?;
    println!("Done! Completed in {:?}", start.elapsed());
    Ok(())
}
