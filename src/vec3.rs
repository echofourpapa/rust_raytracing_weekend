use std::{ops, fmt};

use rand::Rng;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    e: [f64; 3]
}


impl Vec3 {

    pub fn zero() -> Vec3 {
        Vec3{ e: [0.0; 3] }
    }

    pub fn one() -> Vec3 {
        Vec3{e: [1.0; 3] }
    }

    pub fn up() -> Vec3 {
        Vec3{e: [0.0, 1.0, 0.0] }
    }

    pub fn random() -> Vec3 {
        Vec3::new( 
            rand::thread_rng().gen_range(0.0..=1.0),
            rand::thread_rng().gen_range(0.0..=1.0),
            rand::thread_rng().gen_range(0.0..=1.0)
        )
    }   

    pub fn random_range(min:f64, max:f64) -> Vec3 {
        Vec3::new( 
            rand::thread_rng().gen_range(min..=max),
            rand::thread_rng().gen_range(min..=max),
            rand::thread_rng().gen_range(min..=max)
        )
    }   

    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3 { e: [x, y, z] }
    }

    pub fn x(self: &Vec3) -> f64 {
        self.e[0]
    }

    pub fn y(self: &Vec3) -> f64 {
        self.e[1]
    }

    pub fn z(self: &Vec3) -> f64 {
        self.e[2]
    }

    pub fn set_x(self: &mut Vec3, v:f64) {
        self.e[0] = v;
    }

    pub fn set_y(self: &mut Vec3, v:f64) {
        self.e[1] = v;
    }

    pub fn set_z(self: &mut Vec3, v:f64) {
        self.e[2] = v;
    }

    pub fn length(self: &Vec3) -> f64 {
        self.length_squared().sqrt()
    }

    pub fn length_squared(self: &Vec3) -> f64 {
        dot(self, self)
    }

    pub fn normalize(self: &mut Vec3) {
        *self = normalize(*self);
    }

    pub fn sqrt(self: &Vec3) -> Vec3 {
        Vec3::new( 
            self.x().sqrt(),
            self.y().sqrt(),
            self.z().sqrt(),
        )
    }

    pub fn near_zero(self: &Vec3) -> bool {
        self.x().abs() < f64::EPSILON && self.y().abs() < f64::EPSILON && self.z().abs() < f64::EPSILON
    }
}

impl ops::Index<usize> for Vec3 {
        type Output = f64;
        fn index(&self, index: usize) -> &Self::Output {
            &self.e[index]
        }
}

impl ops::IndexMut<usize> for Vec3 {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.e[index]
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3::new( 
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        )
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self =  Vec3::new( 
            self.x() + other.x(),
            self.y() + other.y(),
            self.z() + other.z(),
        );
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3::new( 
            self.x() - other.x(),
            self.y() - other.y(),
            self.z() - other.z(),
        )
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3::new( 
            self.x() * rhs.x(),
            self.y() * rhs.y(),
            self.z() * rhs.z(),
        )
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3::new( 
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        )
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        rhs * self
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64)  {
        *self =         Vec3::new( 
            self.x() * rhs,
            self.y() * rhs,
            self.z() * rhs,
        );
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;
    fn div(self, rhs: f64) -> Vec3 {
        self * (1.0/rhs)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64)  {
        *self *= 1.0/rhs;
    }
}

impl ops::Neg for Vec3 {
    type Output=Vec3;
    fn neg(self) -> Self::Output {
        Vec3::new( 
            -self.x(),
            -self.y(),
            -self.z(),
        )
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x(), self.y(), self.z())
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64{
    u.x()*v.x() + u.y()*v.y() + u.z()*v.z()
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3{
    Vec3::new( 
        u.y() * v.z() - u.z() * v.y(), 
        u.z() * v.x() - u.x() * v.z(), 
        u.x() * v.y() - u.y() * v.x() 
    )
}

pub fn normalize(v: Vec3) -> Vec3 {
    let l: f64 = v.length();
    v / l
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (*n * 2.0 * dot(v,n))
}

pub fn refract(v: &Vec3, n: &Vec3, ior:f64) -> Vec3 {
    let uv: Vec3 = *v;
    let un: Vec3 = *n;
    let cos_theta: f64 = dot(&-uv, &un).min(1.0);
    let r_out_perp: Vec3 = (uv + cos_theta * un ) * ior;
    let r_out_parallel: Vec3 = -(1.0- r_out_perp.length_squared()).abs().sqrt() * un;

    return r_out_perp + r_out_parallel;
}

#[allow(dead_code)]
pub fn lerp(x: &Vec3, y: &Vec3, s: f64) -> Vec3 {
    (*x * (1.0-s)) + (*y * s)
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::new( 
            rand::thread_rng().gen_range(-1.0..=1.0),
            rand::thread_rng().gen_range(-1.0..=1.0),
            rand::thread_rng().gen_range(-1.0..=1.0));
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    return normalize(random_in_unit_sphere());
}

#[allow(dead_code)]
pub fn randon_in_hemisphere(normal: &Vec3) -> Vec3 {
    let rnd_unit: Vec3 = random_unit_vector();
    if dot(&rnd_unit, normal) > 0.0 {
        return rnd_unit;
    } else {
        return -rnd_unit;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p: Vec3 = Vec3::new( 
            rand::thread_rng().gen_range(-1.0..=1.0),
            rand::thread_rng().gen_range(-1.0..=1.0),
             0.0);
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

pub fn linear_to_srgb_f64(c: f64) -> f64 {
    if c <= 0.0031308 { 12.92 * c} else { 1.055 * c.powf(1.0/2.4) - 0.055}
}

impl Color {
    pub fn white() -> Color {
        Color::one()
    }

    pub fn r(self: &Color) -> f64 {
        self.x()
    }

    pub fn g(self: &Color) -> f64 {
        self.y()
    }

    pub fn b(self: &Color) -> f64 {
        self.z()
    }

    pub fn to_srgb(self: &Color) -> Color {
        Color::new(
            linear_to_srgb_f64(self.r()),
            linear_to_srgb_f64(self.g()),
            linear_to_srgb_f64(self.b())
        )
    }
}