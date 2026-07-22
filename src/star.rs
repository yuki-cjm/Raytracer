use std::f64::consts::PI;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::vec3::Point3;

/// A flat 5-pointed star in the XY plane, centered at the origin.
pub struct Star {
    triangles: HittableList,
}

#[allow(dead_code)]
impl Star {
    /// Create a 5-pointed star.
    /// `outer_r` is the tip radius, `inner_r` is the valley radius (~0.38 * outer_r).
    pub fn new(outer_r: f64, inner_r: f64, mat: Arc<dyn Material>) -> Self {
        let mut tris = HittableList::new();
        let thick = 0.03; // tiny thickness so rays don't miss the flat shape

        // Build 10 vertices: alternating outer (tip) then inner (valley)
        let mut verts: Vec<Point3> = Vec::with_capacity(10);
        for i in 0..5 {
            let a = PI / 2.0 + (i as f64) * 2.0 * PI / 5.0;
            verts.push(Point3::new(outer_r * a.cos(), outer_r * a.sin(), 0.0));
            let a = a + PI / 5.0;
            verts.push(Point3::new(inner_r * a.cos(), inner_r * a.sin(), 0.0));
        }

        // Add triangles for front face (+thick) and back face (-thick)
        for i in 0..10 {
            let v1 = &verts[i];
            let v2 = &verts[(i + 1) % 10];

            let c_f = Point3::new(0.0, 0.0, thick);
            let v1_f = Point3::new(v1.x, v1.y, thick);
            let v2_f = Point3::new(v2.x, v2.y, thick);
            let c_b = Point3::new(0.0, 0.0, -thick);
            let v1_b = Point3::new(v1.x, v1.y, -thick);
            let v2_b = Point3::new(v2.x, v2.y, -thick);

            // front (+Z)
            tris.add(Arc::new(Triangle::new(
                &c_f,
                &v1_f,
                &v2_f,
                (0.5, 0.5),
                (0.5 + v1.x / (2.0 * outer_r), 0.5 + v1.y / (2.0 * outer_r)),
                (0.5 + v2.x / (2.0 * outer_r), 0.5 + v2.y / (2.0 * outer_r)),
                mat.clone(),
            )));
            // back (-Z), reversed winding
            tris.add(Arc::new(Triangle::new(
                &c_b,
                &v2_b,
                &v1_b,
                (0.5, 0.5),
                (0.5 + v2.x / (2.0 * outer_r), 0.5 + v2.y / (2.0 * outer_r)),
                (0.5 + v1.x / (2.0 * outer_r), 0.5 + v1.y / (2.0 * outer_r)),
                mat.clone(),
            )));
        }

        Star { triangles: tris }
    }
}

impl Hittable for Star {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        self.triangles.hit(r, ray_t, rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.triangles.bounding_box()
    }
}
