use std::sync::Arc;

use crate::hittable::Hittable;
use crate::onb::Onb;
use crate::rtweekend::{PI, random_double};
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

pub struct MixturePdf<'a> {
    p: [Arc<dyn Pdf + 'a>; 2],
}

impl<'a> MixturePdf<'a> {
    pub fn new(p0: Arc<dyn Pdf + 'a>, p1: Arc<dyn Pdf + 'a>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf<'_> {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            self.p[0].generate()
        } else {
            self.p[1].generate()
        }
    }
}
