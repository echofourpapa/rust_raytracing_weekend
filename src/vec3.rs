use std::ops;

#[derive(Copy, Clone, Default)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}


impl Vec3 {

    pub fn length(self: &Vec3) -> f64{
        self.length_squared().sqrt()
    }

    pub fn length_squared(self: &Vec3) -> f64{
        dot(self, self)
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

pub fn normalize(v: Vec3) -> Vec3{
    let l = v.length();
    v / l
}

pub type Point3 = Vec3;
pub type Color = Vec3;