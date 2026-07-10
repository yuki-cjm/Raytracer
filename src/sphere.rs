use std::rc::Rc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3};

pub struct Sphere {
    pub center: Ray,
    pub radius: f64,
    pub mat: Rc<dyn Material>,
    pub bbox: Aabb,
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time);
        let oc = current_center - r.orig;
        let a = r.dir.length_squared();
        let h = Vec3::dot(&r.dir, &oc);
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = h * h - a * c;
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;
        if !ray_t.surrounds(root) {
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                return false;
            }
        }

        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, &outward_normal);
        rec.mat = Rc::clone(&self.mat);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox.clone()
    }
}

impl Sphere {
    // Stationary Sphere
    pub fn new_stationary(static_center: &Point3, radius: f64, mat: Rc<dyn Material>) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::new(static_center, &Vec3::new(0.0, 0.0, 0.0), 0.0),
            radius: f64::max(0.0, radius),
            mat,
            bbox: Aabb::new_from_points(&(*static_center - rvec), &(*static_center + rvec)),
        }
    }

    // Moving Sphere
    pub fn new_moving(
        center1: &Point3,
        center2: &Point3,
        radius: f64,
        mat: Rc<dyn Material>,
    ) -> Self {
        let center = Ray::new(center1, &(*center2 - *center1), 0.0);
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::new_from_points(&(center.at(0.0) - rvec), &(center.at(0.0) + rvec));
        let box2 = Aabb::new_from_points(&(center.at(1.0) - rvec), &(center.at(1.0) + rvec));
        Self {
            center,
            radius: f64::max(0.0, radius),
            mat,
            bbox: Aabb::new_from_boxs(&box1, &box2),
        }
    }
}
