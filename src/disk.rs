use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::vec3::Point3;

/// A flat circular disk in the XY plane, centered at the origin, facing +Z.
/// UV is mapped so the whole disk gets the full [0,1]×[0,1] texture.
pub struct Disk {
    triangles: HittableList,
}

#[allow(dead_code)]
impl Disk {
    pub fn new(radius: f64, segments: usize, mat: Arc<dyn Material>) -> Self {
        Self::new_with_uv(radius, segments, mat, 0.5, 0.5, 0.5)
    }

    /// `uv_cx`, `uv_cy`: UV center (default 0.5, 0.5).
    /// `uv_r`: UV radius (default 0.5 = inscribed circle of [0,1]×[0,1]).
    ///   Use a smaller value if the texture content doesn't fill the whole inscribed circle.
    pub fn new_with_uv(
        radius: f64,
        segments: usize,
        mat: Arc<dyn Material>,
        uv_cx: f64,
        uv_cy: f64,
        uv_r: f64,
    ) -> Self {
        let mut tris = HittableList::new();
        let thick = 0.02;

        for i in 0..segments {
            let a0 = (i as f64) * 2.0 * std::f64::consts::PI / (segments as f64);
            let a1 = ((i + 1) as f64) * 2.0 * std::f64::consts::PI / (segments as f64);

            let v1 = Point3::new(radius * a0.cos(), radius * a0.sin(), 0.0);
            let v2 = Point3::new(radius * a1.cos(), radius * a1.sin(), 0.0);

            let uv1 = (uv_cx + uv_r * a0.cos(), uv_cy + uv_r * a0.sin());
            let uv2 = (uv_cx + uv_r * a1.cos(), uv_cy + uv_r * a1.sin());
            let uv_c = (uv_cx, uv_cy);

            // front face (+Z)
            let cf = Point3::new(0.0, 0.0, thick);
            let v1f = Point3::new(v1.x, v1.y, thick);
            let v2f = Point3::new(v2.x, v2.y, thick);
            tris.add(Arc::new(Triangle::new(
                &cf,
                &v1f,
                &v2f,
                uv_c,
                uv1,
                uv2,
                mat.clone(),
            )));

            let cb = Point3::new(0.0, 0.0, -thick);
            let v1b = Point3::new(v1.x, v1.y, -thick);
            let v2b = Point3::new(v2.x, v2.y, -thick);
            tris.add(Arc::new(Triangle::new(
                &cb,
                &v2b,
                &v1b,
                uv_c,
                uv2,
                uv1,
                mat.clone(),
            )));
        }

        Disk { triangles: tris }
    }
}

impl Hittable for Disk {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        self.triangles.hit(r, ray_t, rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.triangles.bounding_box()
    }
}
