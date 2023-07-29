use crate::common::*;
use crate::hittable::*;
use crate::ray::*;
use crate::vec3::*;

pub trait Material : Send {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool;
    fn clone_dyn(&self) -> Box<dyn Material + Sync>;
}


impl Clone for Box<dyn Material + Sync> {
    fn clone(&self) -> Self {
        self.clone_dyn()
    }
}

#[derive(Copy, Clone, Default)]
pub struct Lambertian {
    pub albedo: Color
}

#[derive(Copy, Clone, Default)]
pub struct Metal {
    pub albedo: Color,
    pub roughness: f64
}

impl Metal {
    pub fn new(albedo:Color, roughness:f64) -> Metal {
        Metal{albedo:albedo, roughness: Saturate(roughness) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = randon_in_hemisphere(&rec.normal);

        scattered.origin = rec.p;
        scattered.direction = if scatter_direction.near_zero() {rec.normal} else {scatter_direction} ;

        color.x = self.albedo.x;
        color.y = self.albedo.y;
        color.z = self.albedo.z;

        return true;
    }
    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        
        let dir = normalize(ray_in.direction);
        let reflected = reflect(&dir, &rec.normal);       

        scattered.origin = rec.p;
        scattered.direction = reflected + self.roughness * randon_in_hemisphere(&rec.normal);

        color.x = self.albedo.x;
        color.y = self.albedo.y;
        color.z = self.albedo.z;

        return dot(&scattered.direction, &rec.normal) > 0.0;
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}