use std::sync::Arc;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::rtweekend::{PI, random_double};
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Point3, Vec3, dot, reflect, refract};

pub trait Material: Send + Sync {
    fn emitted(&self, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        _attenuation: &mut Color,
        _scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

impl Lambertian {
    pub fn from_color(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        pdf: &mut f64,
    ) -> bool {
        let uvw = Onb::new(&rec.normal);
        let scatter_direction = uvw.transform(&Vec3::random_cosine_direction());

        *scattered = Ray::new(&rec.p, &scatter_direction.unit_vector(), r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        *pdf = dot(&uvw.w(), &scattered.dir) / PI;

        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (2.0 * PI)
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
        _pdf: &mut f64,
    ) -> bool {
        let mut reflected = reflect(&r_in.dir, &rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());
        *scattered = Ray::new(&rec.p, &reflected, r_in.time);
        *attenuation = self.albedo;
        Vec3::dot(&scattered.dir, &rec.normal) > 0.0
    }
}

pub struct Dielectric {
    // Refractive index in vacuum or air, or the ratio of the material's refractive index over
    // the refractive index of the enclosing media
    refraction_index: f64,
}

#[allow(dead_code)]
impl Dielectric {
    pub fn new(refraction_index: f64) -> Dielectric {
        Dielectric { refraction_index }
    }

    fn reflectance(cosine: f64, refraction_index: f64) -> f64 {
        // Use Schlick's approximation for reflectance.
        let r0 = (1.0 - refraction_index) / (1.0 + refraction_index);
        let r0 = r0 * r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);
        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = r_in.dir.unit_vector();
        let cos_theta = f64::min(dot(&unit_direction.neg(), &rec.normal), 1.0);
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;
        let direction =
            if cannot_refract || Dielectric::reflectance(cos_theta, ri) > random_double() {
                reflect(&unit_direction, &rec.normal)
            } else {
                refract(&unit_direction, &rec.normal, ri)
            };

        *scattered = Ray::new(&rec.p, &direction, r_in.time);
        true
    }
}

pub struct DiffuseLight {
    tex: Arc<dyn Texture>,
}

impl DiffuseLight {
    #[allow(dead_code)]
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }

    pub fn from_color(emit: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn emitted(&self, u: f64, v: f64, p: &Point3) -> Color {
        self.tex.value(u, v, p)
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

impl Isotropic {
    pub fn from_color(albedo: &Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    #[allow(dead_code)]
    pub fn new(tex: Arc<dyn Texture>) -> Self {
        Self { tex }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
        _pdf: &mut f64,
    ) -> bool {
        *scattered = Ray::new(&rec.p, &Vec3::random_unit_vector(), r_in.time);
        *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        true
    }
}
