use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, random_double};
use crate::vec3::{Point3, Vec3, cross, dot};

pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    area: f64,
    uv0: (f64, f64),
    uv1: (f64, f64),
    uv2: (f64, f64),
}

impl Triangle {
    pub fn new(
        v0: &Point3,
        v1: &Point3,
        v2: &Point3,
        uv0: (f64, f64),
        uv1: (f64, f64),
        uv2: (f64, f64),
        mat: Arc<dyn Material>,
    ) -> Self {
        let edge1 = *v1 - *v0;
        let edge2 = *v2 - *v0;
        let n = cross(&edge1, &edge2);
        let normal = n.unit_vector();
        let area = n.length() * 0.5;

        // Compute bounding box
        let min = Point3::new(
            f64::min(f64::min(v0.x, v1.x), v2.x),
            f64::min(f64::min(v0.y, v1.y), v2.y),
            f64::min(f64::min(v0.z, v1.z), v2.z),
        );
        let max = Point3::new(
            f64::max(f64::max(v0.x, v1.x), v2.x),
            f64::max(f64::max(v0.y, v1.y), v2.y),
            f64::max(f64::max(v0.z, v1.z), v2.z),
        );
        let bbox = Aabb::new_from_points(&min, &max);

        Self {
            v0: *v0,
            v1: *v1,
            v2: *v2,
            normal,
            mat,
            bbox,
            area,
            uv0,
            uv1,
            uv2,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Möller–Trumbore intersection algorithm
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = cross(&r.dir, &edge2);
        let a = dot(&edge1, &h);

        // Ray is parallel to triangle
        if a.abs() < 1e-8 {
            return false;
        }

        let f = 1.0 / a;
        let s = r.orig - self.v0;
        let u = f * dot(&s, &h);

        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let q = cross(&s, &edge1);
        let v = f * dot(&r.dir, &q);

        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        let t = f * dot(&edge2, &q);

        if !ray_t.contains(t) {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.mat = self.mat.clone();
        rec.set_face_normal(r, &self.normal);
        // Interpolate texture UV from barycentric coordinates
        let w = 1.0 - u - v;
        rec.u = w * self.uv0.0 + u * self.uv1.0 + v * self.uv2.0;
        rec.v = w * self.uv0.1 + u * self.uv1.1 + v * self.uv2.1;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        if !self.hit(
            &Ray::new(origin, direction, 0.0),
            &mut Interval::new(0.001, INFINITY),
            &mut rec,
        ) {
            return 0.0;
        }

        let distance_squared = rec.t * rec.t * direction.length_squared();
        let cosine = dot(direction, &rec.normal).abs() / direction.length();

        distance_squared / (cosine * self.area)
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        // Generate random point on triangle using barycentric coordinates
        let r1 = random_double();
        let r2 = random_double();

        let sqrt_r1 = r1.sqrt();
        let u = 1.0 - sqrt_r1;
        let v = r2 * sqrt_r1;

        let p = self.v0 + u * (self.v1 - self.v0) + v * (self.v2 - self.v0);
        p - *origin
    }
}
