use rand::Rng;

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

#[derive(Copy, Clone, Default)]
pub struct Dielectric {
    pub ior: f64
}

impl Metal {
    pub fn new(albedo:Color, roughness:f64) -> Metal {
        Metal{albedo:albedo, roughness: saturate(roughness) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        let scatter_direction = randon_in_hemisphere(&rec.normal);

        scattered.origin = rec.p;
        scattered.direction = if scatter_direction.near_zero() {rec.normal} else {scatter_direction} ;
        scattered.time = ray_in.time;

        *color = self.albedo;

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
        scattered.time = ray_in.time;

        *color = self.albedo;

        return dot(&scattered.direction, &rec.normal) > 0.0;
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}

fn reflectence(cos:f64, ref_idx:f64) -> f64{
    let mut r0 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0 * r0;
    r0 +(1.0-r0) * (1.0-cos).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {

        *color = Color::white();

        let refraction_ratio = if rec.front_face {1.0/self.ior} else {self.ior};

        let dir = normalize(ray_in.direction);

        let cos_theta = dot(&-dir, &rec.normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let direction;
        let cannot_refract = refraction_ratio * sin_theta > 1.0;
        let rnd_dbl = rand::thread_rng().gen();
        if cannot_refract || reflectence(cos_theta, refraction_ratio) > rnd_dbl {
            direction = reflect(&dir, &rec.normal);
        } else {
            direction = refract(&dir, &rec.normal, refraction_ratio);
        }

        scattered.origin = rec.p;
        scattered.direction = direction;
        scattered.time = ray_in.time;
        
        return true;
    }

    fn clone_dyn(&self) -> Box<dyn Material + Sync> {
        Box::new(self.clone())
    }
}