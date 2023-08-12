use std::{io::{stdout, Write}, sync::{Arc, Mutex}, time::Instant, fs};
use threadpool::ThreadPool;
use rand::Rng;

use crate::{tga, hittable_list::HittableList};
use crate::common::{saturate, seconds_to_hhmmss, degrees_to_radians};
// use crate::world::World;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, Color, normalize, cross, random_in_unit_disk};

#[derive(Copy, Clone)]
pub struct Camera {
    pub origin: Point3,
    pub target: Point3,
    pub up: Point3,
    pub image_width:i32, 
    pub image_height:i32,
    pub samples_per_pixel: i32,
    pub max_depth: i32,
    pub vfov: f64,
    pub defocus_angle: f64,
    pub focus_dist: f64,
    pub delta_time: f64,
    viewport_upper_left: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    pixel00_loc: Vec3,
    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3
    
}

impl Default for Camera {
    fn default() -> Camera {
        Camera {
            origin: Point3::one(),
            target: Point3::zero(),
            up: Point3::up(),
            image_width:1920, 
            image_height:1080,
            samples_per_pixel: 128,
            max_depth: 50,
            vfov: 20.0,
            defocus_angle: 0.1,
            focus_dist: 10.0,
            delta_time: 0.0,
            viewport_upper_left: Point3::zero(),
            pixel_delta_u: Point3::zero(),
            pixel_delta_v: Point3::zero(),
            pixel00_loc: Point3::zero(),
            defocus_disk_u: Point3::zero(),
            defocus_disk_v: Point3::zero()
        }
    }
}

fn write_color(buffer: &mut Vec<u8>, color:&Color, spp: i32, pos:usize) {

    let scale: f64 = 1.0 / (spp as f64);
    let scaled_color: Color = (*color * scale).to_srgb();

    buffer[pos]   =  (255.0 * saturate(scaled_color.b())) as u8;
    buffer[pos+1] =  (255.0 * saturate(scaled_color.g())) as u8;
    buffer[pos+2] =  (255.0 * saturate(scaled_color.r())) as u8;
}

impl Camera {

    pub fn new() -> Camera {
        Camera { ..Default::default() }
    }

    pub fn initialize(self: &mut Camera) {
        let theta: f64 = degrees_to_radians(self.vfov);
        let h: f64 = (theta/2.0).tan();

        let viewport_height: f64 = 2.0 * h * self.focus_dist;
        let viewport_width: f64 = (self.image_width as f64/self.image_height as f64) * viewport_height;

        let w: Vec3 = normalize(self.origin - self.target);
        let u: Vec3 = normalize(cross(&self.up, &w));
        let v: Vec3 = cross(&w, &u);
        
        let viewport_u: Vec3 =  viewport_width * u;
        let viewport_v: Vec3 =  viewport_height * v;

        self.pixel_delta_u = viewport_u / self.image_width as f64;
        self.pixel_delta_v = viewport_v / self.image_height as f64;

        self.viewport_upper_left = self.origin - viewport_u/2.0 - viewport_v/2.0 - (w * self.focus_dist);
        self.pixel00_loc = self.viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius: f64 = self.focus_dist * degrees_to_radians(self.defocus_angle/2.0).tan();
        self.defocus_disk_u = u * defocus_radius;
        self.defocus_disk_v = v * defocus_radius;
    }

    pub fn defocus_disk_sample(&self) -> Point3 {
        let p: Vec3 = random_in_unit_disk();
        self.origin + (p[0] * self.defocus_disk_u) + (p[1] * self.defocus_disk_v)
    }

    pub fn pixel_sample_square(&self) -> Vec3 {
        let px: f64 = -0.5 + rand::thread_rng().gen_range(0.0..=1.0);
        let py: f64 = -0.5 + rand::thread_rng().gen_range(0.0..=1.0);
        (px * self.pixel_delta_u) + (py * self.pixel_delta_v)
    }

    pub fn get_ray(&self, x:i32, y:i32) -> Ray {
        let pixel_center: Vec3 = self.pixel00_loc + (x as f64 * self.pixel_delta_u) + (y as f64 * self.pixel_delta_v);
        let pixel_sample: Vec3 = pixel_center + self.pixel_sample_square();
        let ray_origin: Vec3 = if self.defocus_angle <= 0.0 {self.origin} else {self.defocus_disk_sample()};
        let ray_direction: Vec3 = pixel_sample - ray_origin;

        Ray::new(
            ray_origin,
            ray_direction,
            rand::thread_rng().gen_range(0.0..=self.delta_time)
        )
    }
    
    fn render_pixel(self, x:i32, y: i32, world: &Arc<HittableList>, buffer: &mut Vec<u8>) {
        let mut pixel_color: Vec3 = Color::zero();
        for _s in 0..self.samples_per_pixel {
            let r: Ray = self.get_ray(x, y);
            pixel_color += world.ray_color(&r, self.max_depth);
        }
        let pos: i32 = (x + y * self.image_width) * 3;
        write_color(buffer, &pixel_color, self.samples_per_pixel, pos as usize);
    }

    pub fn render(self, world_arc:&Arc<HittableList>, threads:usize, output:std::path::PathBuf) -> Result<(), std::io::Error> {

        println!("Image size: {}x{}, Samples: {}", self.image_width, self.image_height, self.samples_per_pixel);
        
        let size: i32 = self.image_width * self.image_width * 3;
        let image_buffer: Arc<Mutex<Vec<u8>>> = Arc::new(Mutex::new(vec![0; size as usize]));
    
        // Start timer
        let start: Instant = Instant::now();
        if threads > 1{
            let total_possible_threads: i32 = self.image_height * threads as i32;

            let line_step: i32 = self.image_width / threads as i32;
            let pool: ThreadPool = ThreadPool::new(threads);
            println!("{} threads per scanline, total thread count: {}", threads, total_possible_threads);

            for y in 0..self.image_height {
                for i in 0..threads {
                    pool.execute( {
                        let clone: Arc<Mutex<Vec<u8>>> = Arc::clone(&image_buffer);
                        let world_clone: Arc<HittableList> = world_arc.clone();
                        move || {
                            let scanline_start: i32 = (i as i32 * line_step).min(self.image_width);
                            let scanline_end: i32 = (scanline_start + line_step).min(self.image_width);
                            for x in scanline_start..scanline_end {
                                let mut v: std::sync::MutexGuard<'_, Vec<u8>> = clone.lock().unwrap();
                                self.render_pixel(x, y, &world_clone, &mut v);
                            }
                        }
                    });   
                }
            }
        
            let mut active: usize = pool.active_count();
            let mut queued: usize = pool.queued_count();
            let mut total:usize = active + queued;
            while total > 0 {
                let finished: i32 = total_possible_threads - total as i32;
                let prog: f64 = finished as f64 / total_possible_threads as f64;
                let t: f64 = start.elapsed().as_secs_f64();
                let estimate: f64 = if finished > 0 {(t/finished as f64) * total_possible_threads as f64} else {0.0};
                print!("\r{} Active, {} Remaining, {:.2}% Complete, Running time: {}, Time Remaining {}",
                    active, 
                    queued,
                    prog * 100.0,
                    seconds_to_hhmmss(t),
                    seconds_to_hhmmss(estimate - t)
                );
                stdout().flush().unwrap();
                active = pool.active_count();
                queued = pool.queued_count();
                total = active + queued;
            }
            
            pool.join();
        } else {
            for y in 0..self.image_height {
                for x in 0..self.image_width {
                    let mut v: std::sync::MutexGuard<'_, Vec<u8>> = image_buffer.lock().unwrap();
                    self.render_pixel(x, y, &world_arc, &mut v);

                    let pos: i32 = (x + y * self.image_width) * 3;
                    let prog: f64 = pos as f64 / size as f64;
                    let t: f64 = start.elapsed().as_secs_f64();
                    let estimate: f64 = if pos > 0 {(t/pos as f64) * size as f64} else {0.0};

                    print!("\r{:.2}% Complete, Running time: {}, Time Remaining {}",
                    prog * 100.0,
                    seconds_to_hhmmss(t),
                    seconds_to_hhmmss(estimate - t)
                );
                }
                stdout().flush().unwrap();
            }
        }
        println!("Total render time: {}", seconds_to_hhmmss(start.elapsed().as_secs_f64()));
    
        println!("Saving to: {}", output.display());
        let dir: std::path::PathBuf = output.with_file_name("");
        if !(dir.exists() || dir.as_os_str().is_empty()) {
            fs::create_dir_all(dir)?;
        }
        tga::write_tga_file(self.image_width, self.image_height, &*image_buffer.lock().unwrap(), &output)?;
    
        println!("Total time {}", seconds_to_hhmmss(start.elapsed().as_secs_f64()));
        Ok(())
    }

}