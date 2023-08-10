use std::{thread, sync::Arc};
use camera::Camera;
use clap::Parser;
use clap_num::number_range;

use vec3::Point3;
use world::World;

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


fn demo_scene_range(s: &str) -> Result<u8, String> {
    number_range(s, 0, 1)
}

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args{
    #[arg(short, long, long_help="Output image path.  Only TGA output is supported.", default_value="output/image.tga")]
    output: std::path::PathBuf,

    #[arg(short, long, long_help="Demo scene to render:\n\t0 = Random Spheres\n\t1 = Cornell Box.", value_parser= demo_scene_range, default_value_t=0)]
    demo_scene: i32,

    #[arg(long, long_help="Output image width.", default_value_t=1920)]
    width: i32,

    #[arg(long, long_help="Output image height.", default_value_t=1080)]
    height: i32,

    #[arg(short, long, long_help="Samples per pixel.", default_value_t=50)]
    spp: i32,

    #[arg(short, long, long_help="Max ray bounce depth.", default_value_t=50)]
    max_depth: i32,

    #[arg(short, long, long_help="Max number of threads.", default_value_t=thread::available_parallelism().unwrap().get())]
    threads: usize,
}

fn create_random_world(args: &Args) -> (World, Camera) {
    let world: World = world::random_world();
    let mut cam: Camera = Camera::new();
    cam.origin = Point3::new(13.0, 2.0, 3.0);
    cam.image_width = args.width;
    cam.image_height= args.height;
    cam.samples_per_pixel = args.spp;
    cam.max_depth = args.max_depth;
    cam.initialize();
    (world, cam)
}

fn create_cornell_box(args: &Args) -> (World, Camera) {
    let world: World = world::cornell_box();
    let mut cam: Camera = Camera::new();
    cam.origin = Point3::new(13.0, 2.0, 3.0);
    cam.image_width = args.width;
    cam.image_height= args.height;
    cam.samples_per_pixel = args.spp;
    cam.max_depth = args.max_depth;
    cam.initialize();
    (world, cam)
}

fn error_world() -> (World, Camera) {
    let world: World = World{..Default::default()};
    let cam: Camera = Camera::new();
    println!("Welcome to the void.  If you got here, find out why...");
    (world, cam)
}

fn main() -> Result<(), std::io::Error> {

    let args: Args = Args::parse();

    if !common::validate_path(&args.output) {
        return Ok(());
    }

    let world_cam: (World, Camera) = match args.demo_scene { 
        0=> create_random_world(&args),
        1=> create_cornell_box(&args),
        _=> error_world() // This should never happen, the argument parser should always catch this.
    };
   
    // Camera
    let cam: Camera = world_cam.1;    
    let world_arc: Arc<World> = Arc::new(world_cam.0);

    cam.render(&world_arc, args.threads, args.output)

}
