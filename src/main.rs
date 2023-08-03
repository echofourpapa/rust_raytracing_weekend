use std::{io::{stdout, Write}, sync::{Arc, Mutex}, time::{Instant, Duration}, fs};
use hittable::{HitRecord, Hittable};
use rand::Rng;
use std::thread;
use threadpool::ThreadPool;
use clap::Parser;

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
mod aabb;


fn write_color(buffer: &mut Vec<u8>, color:&Color, spp: u32, pos:usize) {

    let scale: f64 = 1.0 / (spp as f64);
    let scaled_color: Vec3 = (*color as Vec3 * scale).sqrt();

    buffer[pos]   =  (255.0 * saturate(scaled_color.b())) as u8;
    buffer[pos+1] =  (255.0 * saturate(scaled_color.g())) as u8;
    buffer[pos+2] =  (255.0 * saturate(scaled_color.r())) as u8;
}

fn ray_color(r: &Ray, world: &HittableList, depth: i32) -> Color {

    if depth <= 0 {
        return Color::zero();
    }

    let mut rec: HitRecord = HitRecord{..HitRecord::default()};

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


#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args{
    #[arg(long, long_help="Output image path.  Only TGA output is supported.", default_value="output/image.tga")]
    output: std::path::PathBuf,

    #[arg(long, long_help="Output image width.", default_value_t=1920)]
    width: i32,

    #[arg(long, long_help="Output image height.", default_value_t=1080)]
    height: i32,

    #[arg(long, long_help="Samples per pixel.", default_value_t=256)]
    spp: u32,

    #[arg(long, long_help="Max ray bounce depth.", default_value_t=50)]
    max_depth: i32
}

fn validate_path(path: &std::path::PathBuf) -> bool {
    if path.extension().is_none() {
        let suggestion = path.with_extension("tga");
        println!("{} is missing an extension. Did you mean {}?", path.display(), suggestion.display());
        return false;
    } else {
        if path.extension().unwrap().to_ascii_lowercase() != "tga" {
            println!("Only TGA format is supported.");
            return false;
        }
    }
    path.has_root() || path.is_relative()
}

fn main() -> Result<(), std::io::Error> {

    let args = Args::parse();

    if !validate_path(&args.output) {
        return Ok(());
    }

    // Image    
    let image_width: i32 = args.width;
    let image_height: i32 = args.height;
    let aspect_ratio: f64 = image_width as f64 / image_height as f64;
    let samples_per_pixel: u32 = args.spp;
    let max_depth: i32 = args.max_depth;

    // World
    let world: HittableList = random_world();
   
    // Camera
    let cam_origin: Vec3 = Point3::new(13.0, 2.0, 3.0);
    let cam_target: Vec3 = Point3::new(0.0, 0.0, 0.0);

    let cam: Camera = Camera::new(
        cam_origin,
        cam_target, 
        Vec3::up(), 
        20.0, 
        aspect_ratio,
        0.1,
        10.0,
        1.0
    );

    let max_threads: usize = thread::available_parallelism().unwrap().get() - 1;
    let pool = ThreadPool::new(max_threads);

    println!("Image size: {}x{}", image_width, image_height);
    let size: i32 = image_width * image_width * 3;
    let image_buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; size as usize]));

    let world_arc: Arc<HittableList> = Arc::new(world);

    let line_step: i32 = image_width / max_threads as i32;

    // Start timer
    let start: Instant = Instant::now();
    let _st: f64 = start.elapsed().as_secs_f64();

    for y in 0..image_height {
        for i in 0..line_step {
            pool.execute( {
                let clone: Arc<Mutex<Vec<u8>>> = Arc::clone(&image_buffer);
                let world_clone: Arc<HittableList> = world_arc.clone();
                move || {
                    let scanline_start: i32 = (i * line_step).min(image_width);
                    let scanline_end: i32 = (scanline_start + line_step).min(image_width);
                    for x in scanline_start..scanline_end {
                        let mut pixel_color: Vec3 = Color::zero();
                        for _s in 0..samples_per_pixel {
                            let a: f64 = rand::thread_rng().gen_range(0.0..=1.0);
                            let b: f64 = rand::thread_rng().gen_range(0.0..=1.0);
                            let u: f64 = (x as f64 + a) / ((image_width-1) as f64);
                            let v: f64 = (y as f64 + b) / ((image_height-1) as f64);
                            let r: Ray = cam.get_ray(u, v);
                            pixel_color += ray_color(&r, &world_clone, max_depth);
                        }
                        let pos: i32 = (x + y * image_width) * 3;
                        let mut v: std::sync::MutexGuard<'_, Vec<u8>> = clone.lock().unwrap();
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
        let t: f64 = start.elapsed().as_secs_f64();
        let estimate: f64 = if finished > 0 {(t/finished as f64) * total_possible_threads as f64} else {0.0};
        print!("\r{} Active, {} Queued, {} total, {:.2}% Complete, Running time: {}, Time Remaining {}",
            pool.active_count(), 
            pool.queued_count(),
            total_possible_threads,
            prog * 100.0,
            seconds_to_hhmmss(t),
            seconds_to_hhmmss(estimate - t)
        );
        stdout().flush().unwrap();

    }
    
    pool.join();

    print!("\n");
    println!("Saving to: {}", args.output.display());

    let dir: std::path::PathBuf = args.output.with_file_name("");
    if !(dir.exists() || dir.as_os_str().is_empty()) {
        fs::create_dir_all(dir)?
    }

    tga::write_tga_file(image_width, image_height, &*image_buffer.lock().unwrap(), &args.output)?;
    println!("Done! Completed in {}", seconds_to_hhmmss(start.elapsed().as_secs_f64()));
    Ok(())
}
