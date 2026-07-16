use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend::PI;
use crate::vec3::{Point3, Vec3, dot};

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

#[allow(dead_code)]
impl CosinePdf {
    pub fn new(w: &Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = dot(&direction.unit_vector(), &self.uvw.w());
        f64::max(0.0, cosine_theta / PI)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(&Vec3::random_cosine_direction())
    }
}

pub struct HittablePdf<'a> {
    objects: &'a (dyn Hittable + Sync),
    origin: Point3,
}

impl<'a> HittablePdf<'a> {
    pub fn new(objects: &'a (dyn Hittable + Sync), origin: &Point3) -> Self {
        Self {
            objects,
            origin: *origin,
        }
    }
}

impl Pdf for HittablePdf<'_> {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}
