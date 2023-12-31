use std::sync::Arc;

use rand::Rng;

use crate::common::*;
use crate::hittable::*;
use crate::ray::*;
use crate::texture::SolidColorTexture;
use crate::texture::Texture;
use crate::vec3::*;

pub trait Material : Send {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool;
    fn emitted(&self, rec: &HitRecord) -> Color;
}

#[derive(Clone, Default)]
pub struct Lambertian {
    pub albedo: Option<Arc<dyn Texture + Sync>>
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

#[derive(Copy, Clone, Default)]
pub struct Emiter {
    pub emission: Color
}

impl Lambertian {
    pub fn new(albedo: Color) -> Lambertian {
        Lambertian {
            albedo: Some(Arc::new(SolidColorTexture{color: albedo}))
        }
    }

    pub fn new_texture(albedo: &Arc<dyn Texture + Sync>) -> Lambertian {
        Lambertian {
            albedo: Some(albedo.to_owned())
        }
    }
}

impl Metal {
    pub fn new(albedo:Color, roughness:f64) -> Metal {
        Metal{albedo:albedo, roughness: saturate(roughness) }
    }
}

impl Material for Lambertian {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        let mut scatter_direction: Vec3 = random_unit_vector() + rec.normal;

        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }

        *scattered = Ray::new(rec.p, scatter_direction, ray_in.time);
        *color = self.albedo.as_ref().unwrap().value(rec.uvw.x(), rec.uvw.y(), rec.uvw.z());

        return true;
    }

    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::black()
    }
}

impl Material for Metal {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {
        
        let dir = normalize(ray_in.direction);
        let reflected = reflect(&dir, &rec.normal);       

        *scattered = Ray::new(rec.p, reflected + self.roughness * random_unit_vector(), ray_in.time);
        *color = self.albedo;

        return dot(&scattered.direction, &rec.normal) > 0.0;
    }

    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::black()
    }
}

fn reflectence(cos:f64, ref_idx:f64) -> f64{
    let mut r0: f64 = (1.0-ref_idx) / (1.0+ref_idx);
    r0 = r0 * r0;
    r0 +(1.0-r0) * (1.0-cos).powf(5.0)
}

impl Material for Dielectric {
    fn scatter(&self, ray_in: &Ray, rec: &HitRecord, color: &mut Color, scattered: &mut Ray) -> bool {

        *color = Color::white();

        let refraction_ratio: f64 = if rec.front_face {1.0/self.ior} else {self.ior};

        let dir: Vec3 = normalize(ray_in.direction);

        let cos_theta: f64 = dot(&-dir, &rec.normal).min(1.0);
        let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract: bool = refraction_ratio * sin_theta > 1.0;
        let rnd_dbl: f64 = rand::thread_rng().gen();

        let should_reflect: bool = cannot_refract || reflectence(cos_theta, refraction_ratio) > rnd_dbl;

        let direction: Vec3 = if should_reflect { reflect(&dir, &rec.normal) } else { refract(&dir, &rec.normal, refraction_ratio) };

        *scattered = Ray::new(rec.p, direction, ray_in.time);
        
        return true;
    }

    fn emitted(&self, _rec: &HitRecord) -> Color {
        Color::black()
    }
}

impl Material for Emiter {
    fn scatter(&self, _ray_in: &Ray, _rec: &HitRecord, _color: &mut Color, _scattered: &mut Ray) -> bool {
        false
    }

    fn emitted(&self, rec: &HitRecord) -> Color {
        if rec.front_face{
            self.emission
        } else {
            Color::black()
        }
    }
}