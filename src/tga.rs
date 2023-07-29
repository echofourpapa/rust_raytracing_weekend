use std::fs::File;
use std::io::Write;
use std::mem;
use std::path::PathBuf;
use std::slice;

unsafe fn struct_to_u8_slice<T>(s: &T) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(s);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>())
}

unsafe fn slice_to_u8_slice<T>(s: &[T]) -> &[u8] {
    let data_ptr: *const u8 = mem::transmute(&s[0]);
    slice::from_raw_parts(data_ptr, mem::size_of::<T>() * s.len())
}

#[repr(C, packed)]
#[derive(Default)]
struct tga_color_map_spec {
    map_start: u16,
    map_length: u16,
    map_depth: u8,
}

#[repr(C, packed)]
#[derive(Default)]
struct tga_image_spec{
    x_origin: u16,
    y_origin: u16,
    image_width: u16,
    image_height: u16,
    pixel_depth: u8,
    descriptor: u8,
}

#[repr(C, packed)]
#[derive(Default)]
struct tga_header{
    id_legnth :u8,
    color_map_type : u8,
    image_type : u8,
    color_map_spec: tga_color_map_spec,
    image_spec : tga_image_spec,
}

fn get_tga_header(width: i32, height: i32) -> tga_header {

    let color_spec = tga_color_map_spec{
        ..tga_color_map_spec::default()
    };

    let img_spec = tga_image_spec {
        image_width: width as u16,
        image_height: height as u16,
        pixel_depth: 24,
        ..tga_image_spec::default()
    };

    let header = tga_header{
        image_type: 2,
        color_map_spec: color_spec,
        image_spec: img_spec,
        ..tga_header::default()
    };

    header
}

pub fn write_tga_file(width: i32, height: i32, image_data: &Vec<u8>, file_path: &PathBuf) -> Result<(), std::io::Error> {

    let mut file = File::create(file_path)?;

    let header = get_tga_header(width, height);
    let header_bytes: &[u8] = unsafe{ struct_to_u8_slice(&header) };

    file.write_all(header_bytes)?;
    file.write_all(image_data)?;

    Ok(())
}