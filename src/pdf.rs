use crate::onb::Onb;
use crate::rtweekend::PI;
use crate::vec3::{Vec3, dot};

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;

    fn generate(&self) -> Vec3;
}

pub struct CosinePdf {
    uvw: Onb,
}

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
