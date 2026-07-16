use std::sync::Arc;

use crate::color::Color;
use crate::hittable::HitRecord;
use crate::pdf::{CosinePdf, EmptyPdf, Pdf, SpherePdf};
use crate::ray::Ray;
use crate::rtweekend::{PI, random_double};
use crate::texture::{SolidColor, Texture};
use crate::vec3::{Point3, Vec3, dot, reflect, refract};

pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Arc<dyn Pdf>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Ray,
}

impl Default for ScatterRecord {
    fn default() -> Self {
        Self {
            attenuation: Color::default(),
            pdf_ptr: Arc::new(EmptyPdf),
            skip_pdf: false,
            skip_pdf_ray: Ray::default(),
        }
    }
}

pub trait Material: Send + Sync {
    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scatter(&self, _r_in: &Ray, _rec: &HitRecord, _srec: &mut ScatterRecord) -> bool {
        false
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

pub struct EmptyMaterial;
impl Material for EmptyMaterial {}

#[derive(Clone)]
pub struct Lambertian {
    tex: Arc<dyn Texture>,
}

#[allow(dead_code)]
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(CosinePdf::new(&rec.normal));
        srec.skip_pdf = false;

        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        let cos_theta = dot(&rec.normal, &scattered.dir.unit_vector());
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
    }
}

#[derive(Clone)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

#[allow(dead_code)]
impl Metal {
    pub fn new(albedo: &Color, fuzz: f64) -> Metal {
        Metal {
            albedo: *albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        let mut reflected = reflect(&r_in.dir, &rec.normal);
        reflected = reflected.unit_vector() + (self.fuzz * Vec3::random_unit_vector());

        srec.attenuation = self.albedo;
        srec.pdf_ptr = Arc::new(EmptyPdf);
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Ray::new(&rec.p, &reflected, r_in.time);

        true
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
    fn scatter(&self, r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = Arc::new(EmptyPdf);
        srec.skip_pdf = true;
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

        srec.skip_pdf_ray = Ray::new(&rec.p, &direction, r_in.time);
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
    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        if !rec.front_face {
            Color::new(0.0, 0.0, 0.0)
        } else {
            self.tex.value(u, v, p)
        }
    }
}

pub struct Isotropic {
    tex: Arc<dyn Texture>,
}

#[allow(dead_code)]
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
    fn scatter(&self, _r_in: &Ray, rec: &HitRecord, srec: &mut ScatterRecord) -> bool {
        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Arc::new(SpherePdf);
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
