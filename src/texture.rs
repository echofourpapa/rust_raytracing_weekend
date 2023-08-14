use std::path::PathBuf;

use crate::{vec3::Color, tga::read_tga_file, common::saturate};

#[derive(Copy, Clone, Default)]
pub struct SolidColorTexture {
    pub color: Color
}

pub trait Texture : Send {
    fn value(&self, u:f64, v:f64, w:f64) -> Color;
}

impl Texture for SolidColorTexture {
    fn value(&self, _u:f64, _v:f64, _w:f64) -> Color {
        return self.color;
    }
}

#[derive(Copy, Clone, Default)]
pub struct CheckerTexture {
    pub color_a: Color,
    pub color_b: Color,
    pub scale: f64
}

impl CheckerTexture {
   pub fn new(a:Color, b:Color, scale:f64) -> CheckerTexture {
        CheckerTexture { 
            color_a: a,
            color_b: b,
            scale: scale 
        }
   } 
}

impl Texture for CheckerTexture {
    fn value(&self, u:f64, v:f64, w:f64) -> Color {
        let x: i32 = (u * self.scale).floor() as i32;
        let y: i32 = (v * self.scale).floor() as i32;
        let z: i32 = (w * self.scale).floor() as i32;
        let is_even: bool = (x+y+z) % 2 == 0;
        if is_even {
            return self.color_a;
        } else {
            return self.color_b;
        }
    }
}

#[derive(Clone, Default)]
pub struct ImageTexture {
    pub bytes_per_pixel: usize,
    pub width: usize,
    pub height: usize,
    pub data: Vec<u8>,
    pub bytes_per_scanline: usize
}

impl ImageTexture {
    pub fn new(file_path: &PathBuf) -> ImageTexture {
        let mut img: ImageTexture = ImageTexture::default();
        let _ = read_tga_file(file_path, 
            &mut img.data, 
            &mut img.width, 
            &mut img.height, 
            &mut img.bytes_per_pixel
        );

        img.bytes_per_scanline = img.width * img.bytes_per_pixel;
        
        img
    }
}

impl Texture for ImageTexture {
    fn value(&self, u:f64, v:f64, _w:f64) -> Color {
        if self.height == 0 {
            return Color::new(1.0,0.0, 1.0);
        }

        let s_u: f64 = saturate(u);
        let s_v: f64 = saturate(v);

        let x: usize = (s_u * self.width as f64) as usize;
        let y: usize = (s_v * self.height as f64) as usize;
        let pos: usize = x * self.bytes_per_pixel + y * self.bytes_per_scanline;
        let mut color: Color = Color::black();

        color[0] = self.data[pos + 2] as f64 / 255.0;
        color[1] = self.data[pos + 1] as f64 / 255.0;
        color[2] = self.data[pos + 0] as f64 / 255.0;

        color
    }
}