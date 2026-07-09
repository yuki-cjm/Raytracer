use crate::color::Color;
use crate::hittable::HitRecord;
use crate::ray::Ray;
use crate::vec3::{Vec3, reflect};

pub trait Material {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

#[derive(Clone)]
pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: &Color) -> Lambertian {
        Lambertian { albedo: *albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::random_unit_vector();

        // Catch degenerate scatter direction
        if scatter_direction.near_zero() {
            scatter_direction = rec.normal;
        }
        *scattered = Ray::new(&rec.p, &scatter_direction);
        *attenuation = self.albedo;
        true
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Metal {
        Metal {
            albedo: *albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut reflected = reflect(&r_in.dir, &rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(&rec.p, &reflected);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.dir, &rec.normal) > 0.0
    }
}
