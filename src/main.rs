use std::{thread, sync::Arc};
use camera::Camera;
use clap::Parser;
use clap_num::number_range;

use hittable::Hittable;
use vec3::Point3;

use crate::{hittable_list::HittableList, vec3::Color};


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
mod world;
mod bvh;
mod interval;
mod quad;
mod texture;


fn demo_scene_range(s: &str) -> Result<i32, String> {
    number_range(s, 0, 2)
}

fn thread_range(s: &str) -> Result<usize, String> {
    number_range(s, 1, thread::available_parallelism().unwrap().get())
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args{
    #[arg(short, long, long_help="Output image path.  Only TGA output is supported.", default_value="output/image.tga")]
    output: std::path::PathBuf,

    #[arg(short, long, long_help="Demo scene to render:\n\t0 = Random Spheres\n\t1 = Cornell Box.\n\t2 = Qauds", value_parser=demo_scene_range, default_value_t=1)]
    demo_scene: i32,

    #[arg(long, long_help="Output image width.", default_value_t=1920)]
    width: i32,

    #[arg(long, long_help="Output image height.", default_value_t=1080)]
    height: i32,

    #[arg(short, long, long_help="Samples per pixel.", default_value_t=256)]
    spp: i32,

    #[arg(short, long, long_help="Max ray bounce depth.", default_value_t=50)]
    max_depth: i32,

    #[arg(short, long, long_help="Max number of threads. 1 means disable threading.", value_parser=thread_range, default_value_t=thread::available_parallelism().unwrap().get())]
    threads: usize,
}

fn create_random_world(args: &Args) -> (HittableList, Camera) {
    println!("Setting up random sphere's scene.");
    let world: HittableList = world::random_world();
    let mut cam: Camera = Camera::new();
    cam.origin = Point3::new(13.0, 2.0, 3.0);
    cam.image_width = args.width;
    cam.image_height= args.height;
    cam.samples_per_pixel = args.spp;
    cam.max_depth = args.max_depth;
    cam.background = Color::new(0.7, 0.8, 1.0);
    cam.initialize();
    (world, cam)
}

fn create_cornell_box(args: &Args) -> (HittableList, Camera) {
    println!("Setting up Cornell Box.");
    let world: HittableList = world::cornell_box();
    let mut cam: Camera = Camera::new();
    cam.origin = Point3::new(278.0, 278.0, -800.0);
    cam.target = Point3::new(278.0, 278.0, 0.0);
    cam.vfov = 40.0;
    cam.defocus_angle = 0.0;
    cam.image_width = args.width;
    cam.image_height= args.height;
    cam.samples_per_pixel = args.spp;
    cam.max_depth = args.max_depth;
    cam.background = Color::new(0.7, 0.8, 1.0);
    cam.initialize();
    (world, cam)
}

fn create_quads(args: &Args) -> (HittableList, Camera) {
    println!("Setting up quad scene.");
    let world: HittableList = world::quads();
    let mut cam: Camera = Camera::new();
    cam.origin = Point3::new(0.0, 0.0, 9.0);
    cam.vfov = 80.0;
    cam.defocus_angle = 0.0;
    cam.image_width = args.width;
    cam.image_height= args.height;
    cam.samples_per_pixel = args.spp;
    cam.max_depth = args.max_depth;
    cam.background = Color::new(0.7, 0.8, 1.0);
    cam.initialize();
    (world, cam)
}

fn error_world() -> (HittableList, Camera) {
    let world: HittableList = HittableList{..Default::default()};
    let cam: Camera = Camera::new();
    println!("Welcome to the void.  If you got here, find out why...");
    (world, cam)
}

fn main() -> Result<(), std::io::Error> {

    let args: Args = Args::parse();

    let mut output_path: std::path::PathBuf = args.output.clone();
    if !common::validate_path(&mut output_path) {
        return Ok(());
    }

    let world_cam: (HittableList, Camera) = match args.demo_scene { 
        0=> create_random_world(&args),
        1=> create_cornell_box(&args),
        2=> create_quads(&args),
        _=> error_world() // This should never happen, the argument parser should always catch this.
    };
   
    // Camera
    let cam: Camera = world_cam.1;    
    let world_arc: Arc<HittableList> = Arc::new(world_cam.0);
    
    cam.render(&(world_arc as Arc<dyn Hittable + Sync>), args.threads, output_path)

}
