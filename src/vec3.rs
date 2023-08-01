use std::{ops, fmt};

use rand::Rng;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}


impl Vec3 {

    pub fn zero() -> Vec3 {
        Vec3{x:0.0, y:0.0, z:0.0 }
    }

    pub fn one() -> Vec3 {
        Vec3{x:1.0, y:1.0, z:1.0 }
    }

    pub fn up() -> Vec3 {
        Vec3{x:0.0, y:1.0, z:0.0 }
    }

    pub fn random() -> Vec3 {
        Vec3{ 
            x:rand::thread_rng().gen_range(0.0..=1.0),
            y:rand::thread_rng().gen_range(0.0..=1.0),
            z:rand::thread_rng().gen_range(0.0..=1.0)
        }
    }   

    pub fn random_range(min:f64, max:f64) -> Vec3 {
        Vec3{ 
            x:rand::thread_rng().gen_range(min..=max),
            y:rand::thread_rng().gen_range(min..=max),
            z:rand::thread_rng().gen_range(min..=max)
        }
    }   

    pub fn new(x:f64, y:f64, z:f64) -> Vec3 {
        Vec3 { x:x, y: y, z: z }
    }

    pub fn x(self: &Vec3) -> f64 {
        self.x
    }

    pub fn y(self: &Vec3) -> f64 {
        self.y
    }

    pub fn z(self: &Vec3) -> f64 {
        self.z
    }

    pub fn set_x(self: &mut Vec3, v:f64) {
        self.x = v;
    }

    pub fn set_y(self: &mut Vec3, v:f64) {
        self.y = v;
    }

    pub fn set_z(self: &mut Vec3, v:f64) {
        self.z = v;
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

    pub fn near_zero(self: &Vec3) -> bool {
        self.x.abs() < f64::EPSILON && self.y.abs() < f64::EPSILON && self.z.abs() < f64::EPSILON
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;
    fn add(self, other: Vec3) -> Vec3 {
        Vec3{ 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, other: Vec3) {
        *self =  Vec3{ 
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        };
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;
    fn sub(self, other: Vec3) -> Vec3 {
        Vec3{ 
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: Vec3) -> Vec3 {
        Vec3{ 
            x: self.x * rhs.x,
            y: self.y * rhs.y,
            z: self.z * rhs.z,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;
    fn mul(self, rhs: f64) -> Vec3 {
        Vec3{ 
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
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
        *self = Vec3{ 
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        };
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
        Vec3{
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}

pub fn dot(u: &Vec3, v: &Vec3) -> f64{
    u.x*v.x + u.y*v.y + u.z*v.z
}

pub fn cross(u: &Vec3, v: &Vec3) -> Vec3{
    Vec3 { 
        x: u.y * v.z - u.z * v.y, 
        y: u.z * v.x - u.x * v.z, 
        z: u.x * v.y - u.y * v.x 
    }
}

pub fn normalize(v: Vec3) -> Vec3 {
    let l = v.length();
    v / l
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - (*n * 2.0 * dot(v,n))
}

pub fn refract(v: &Vec3, n: &Vec3, ior:f64) -> Vec3 {
    let cos_theta = dot(&-*v, n).min(1.0);
    let r_out_perp = (*v + cos_theta * *n ) * ior;
    let g = 1.0- r_out_perp.length_squared();
    let h = g.abs().sqrt();
    let r_out_parallel = -h * *n;

    return r_out_perp + r_out_parallel;
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        let p = Vec3{ 
            x:rand::thread_rng().gen_range(-1.0..=1.0),
            y:rand::thread_rng().gen_range(-1.0..=1.0),
            z:rand::thread_rng().gen_range(-1.0..=1.0)};
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_unit_vector() -> Vec3 {
    return normalize(random_in_unit_sphere());
}

pub fn randon_in_hemisphere(normal: &Vec3) -> Vec3 {
    let rnd_unit = random_unit_vector();
    if dot(&rnd_unit, normal) > 0.0 {
        return rnd_unit;
    } else {
        return -rnd_unit;
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3{ 
            x:rand::thread_rng().gen_range(-1.0..=1.0),
            y:rand::thread_rng().gen_range(-1.0..=1.0),
            z: 0.0};
        if p.length_squared() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub type Point3 = Vec3;
pub type Color = Vec3;

impl Color {
    pub fn white() -> Color {
        Vec3::one()
    }
}