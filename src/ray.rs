use crate::vec3::{Point3, Vec3};

pub struct Ray {
    #[allow(dead_code)]
    pub orig: Point3,
    pub dir: Vec3,
}

impl Ray {
    pub fn new(origin: &Point3, direction: &Vec3) -> Self {
        Self {
            orig: *origin,
            dir: *direction,
        }
    }

    #[allow(dead_code)]
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + self.dir * t
    }
}
