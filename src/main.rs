use std::{thread, sync::Arc};
use camera::Camera;
use clap::Parser;

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

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args{
    #[arg(long, long_help="Output image path.  Only TGA output is supported.", default_value="output/image.tga")]
    output: std::path::PathBuf,

    #[arg(long, long_help="Output image width.", default_value_t=1920)]
    width: i32,

    #[arg(long, long_help="Output image height.", default_value_t=1080)]
    height: i32,

    #[arg(long, long_help="Samples per pixel.", default_value_t=50)]
    spp: i32,

    #[arg(long, long_help="Max ray bounce depth.", default_value_t=50)]
    max_depth: i32,

    #[arg(long, long_help="Max number of threads.", default_value_t=thread::available_parallelism().unwrap().get())]
    threads: usize,

    #[arg(long, long_help="Output progress", default_value_t=false)]
    silent: bool
}

fn main() -> Result<(), std::io::Error> {

    let args: Args = Args::parse();

    if !common::validate_path(&args.output) {
        return Ok(());
    }

    // World
    let world: World = world::random_world();
   
    // Camera
    let mut cam: Camera = Camera::new();
    cam.origin = Point3::new(13.0, 2.0, 3.0);
    cam.image_width = args.width;
    cam.image_height= args.height;
    cam.samples_per_pixel = args.spp;
    cam.max_depth = args.max_depth;


    cam.initialize();
    let world_arc: Arc<World> = Arc::new(world);
    cam.render(&world_arc, args.threads, args.output)

}
