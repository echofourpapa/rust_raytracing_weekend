use crate::vec3::Color;

#[derive(Copy, Clone, Default)]
pub struct SolidColorTexture {
    pub color: Color
}

pub trait Texture : Send {
    fn value(&self, u:f64, v:f64) -> Color;
}

impl Texture for SolidColorTexture {
    fn value(&self, _u:f64, _v:f64) -> Color {
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
    fn value(&self, u:f64, v:f64) -> Color {
        let x: i32 = (u * self.scale).floor() as i32;
        let y: i32 = (v * self.scale).floor() as i32;
        let is_even: bool = (x+y) % 2 == 0;
        if is_even {
            return self.color_a;
        } else {
            return self.color_b;
        }
    }
}