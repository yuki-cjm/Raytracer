use crate::vec3::{Point3, Vec3};

#[derive(Default)]
pub struct Ray {
    pub orig: Point3,
    pub dir: Vec3,
    pub time: f64, // 0.0
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3, time: f64) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
            time,
        }
    }

    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
