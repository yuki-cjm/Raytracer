use std::sync::Arc;

use crate::aabb::Aabb;
use crate::color::Color;
use crate::interval::Interval;
use crate::material::{Lambertian, Material};
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, degrees_to_radians};
use crate::vec3::{Point3, Vec3};

#[derive(Clone)]
pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub mat: Arc<dyn Material>,
    pub t: f64,
    pub u: f64,
    pub v: f64,
    pub front_face: bool,
}

pub trait Hittable: Send + Sync {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool;

    fn bounding_box(&self) -> Aabb;

    fn pdf_value(&self, _origin: &Point3, _direction: &Vec3) -> f64 {
        0.0
    }

    fn random(&self, _origin: &Point3) -> Vec3 {
        Vec3::new(1.0, 0.0, 0.0)
    }
}

impl HitRecord {
    pub fn default() -> Self {
        Self {
            p: Point3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            mat: Arc::new(Lambertian::from_color(&Color::new(0.0, 0.0, 0.0))),
            t: 0.0,
            u: f64::default(),
            v: f64::default(),
            front_face: false,
        }
    }

    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: &Vec3) {
        // Sets the hit record normal vector.
        // NOTE: the parameter `outward_normal` is assumed to have unit length.

        self.front_face = Vec3::dot(&r.dir, outward_normal) < 0.0;
        self.normal = if self.front_face {
            *outward_normal
        } else {
            *outward_normal * (-1.0)
        };
    }
}

pub struct Translate {
    object: Arc<dyn Hittable>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable>, offset: &Vec3) -> Self {
        let bbox = object.bounding_box() + *offset;
        Self {
            object,
            offset: *offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        // Move the ray backwards by the offset
        let offset_r = Ray::new(&(r.orig - self.offset), &r.dir, r.time);

        // Determine whether an intersection exists along the offset ray (and if so, where)
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        // Move the intersection point forwards by the offset
        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

#[allow(dead_code)]
impl RotateY {
    pub fn new(object: Arc<dyn Hittable>, angle: f64) -> Self {
        let radians = degrees_to_radians(angle);
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = i as f64 * bbox.x.max + (1.0 - i as f64) * bbox.x.min;
                    let y = j as f64 * bbox.y.max + (1.0 - j as f64) * bbox.y.min;
                    let z = k as f64 * bbox.z.max + (1.0 - k as f64) * bbox.z.min;

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    min.x = f64::min(min.x, tester.x);
                    max.x = f64::max(max.x, tester.x);
                    min.y = f64::min(min.y, tester.y);
                    max.y = f64::max(max.y, tester.y);
                    min.z = f64::min(min.z, tester.z);
                    max.z = f64::max(max.z, tester.z);
                }
            }
        }

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox: Aabb::new_from_points(&min, &max),
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let cos_theta = self.cos_theta;
        let sin_theta = self.sin_theta;

        // Transform the ray from world space to object space.

        let origin = Vec3::new(
            cos_theta * r.orig.x - sin_theta * r.orig.z,
            r.orig.y,
            sin_theta * r.orig.x + cos_theta * r.orig.z,
        );

        let direction = Vec3::new(
            cos_theta * r.dir.x - sin_theta * r.dir.z,
            r.dir.y,
            sin_theta * r.dir.x + cos_theta * r.dir.z,
        );

        let rotated_r = Ray::new(&origin, &direction, r.time);

        // Determine whether an intersection exists in object space (and if so, where).

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        // Transform the intersection from object space back to world space.

        rec.p = Point3::new(
            cos_theta * rec.p.x + sin_theta * rec.p.z,
            rec.p.y,
            -sin_theta * rec.p.x + cos_theta * rec.p.z,
        );

        rec.normal = Vec3::new(
            cos_theta * rec.normal.x + sin_theta * rec.normal.z,
            rec.normal.y,
            -sin_theta * rec.normal.x + cos_theta * rec.normal.z,
        );

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct Scale {
    object: Arc<dyn Hittable>,
    scale: f64,
    bbox: Aabb,
}

impl Scale {
    pub fn new(object: Arc<dyn Hittable>, scale: f64) -> Self {
        let bbox = object.bounding_box();
        let min = Point3::new(bbox.x.min * scale, bbox.y.min * scale, bbox.z.min * scale);
        let max = Point3::new(bbox.x.max * scale, bbox.y.max * scale, bbox.z.max * scale);
        let bbox = Aabb::new_from_points(&min, &max);
        Self {
            object,
            scale,
            bbox,
        }
    }
}

impl Hittable for Scale {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        let inv_scale = 1.0 / self.scale;

        // Transform the ray to object space (where the object is unscaled)
        let scaled_r = Ray::new(&(r.orig * inv_scale), &r.dir, r.time);

        // Also scale ray_t to object space
        let mut scaled_ray_t = *ray_t;
        scaled_ray_t.min *= inv_scale;
        scaled_ray_t.max *= inv_scale;

        if !self.object.hit(&scaled_r, &mut scaled_ray_t, rec) {
            return false;
        }

        // Transform intersection back to world space
        rec.p *= self.scale;
        rec.t *= self.scale;

        // Normal direction is unchanged for uniform scale
        rec.normal = rec.normal.unit_vector();

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
