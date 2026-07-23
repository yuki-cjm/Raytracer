use std::sync::Arc;

use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::hittable_list::HittableList;
use crate::interval::Interval;
use crate::material::Material;
use crate::ray::Ray;
use crate::triangle::Triangle;
use crate::vec3::Point3;

/// A flat crescent moon shape, facing +Z, with the outer-arc midpoint
/// at the top so it can hang from a string (horns point downward).
///
/// The crescent is the set-difference of two circles:
/// an outer circle (radius `outer_r`, center at origin) minus
/// an inner circle (radius `inner_r`, center at (`offset_x`, 0)).
///
/// Triangulation uses a triangle strip between the outer and inner arcs
/// (instead of a centre fan, which fails when the centre lies inside the
/// subtracted inner circle).
pub struct Crescent {
    triangles: HittableList,
}

#[allow(dead_code)]
impl Crescent {
    /// Create a crescent moon that hangs with its horns pointing down.
    ///
    /// * `outer_r` – radius of the outer (convex) arc.
    /// * `inner_r` – radius of the inner (concave) arc.
    ///   For a thin crescent, use `inner_r` slightly larger than `outer_r`.
    /// * `offset_x` – horizontal offset of the inner circle centre.
    ///   Together with the radii this controls crescent thickness.
    /// * `segments` – subdivisions per arc (higher = smoother).
    ///
    /// Example for a thin elegant crescent:
    /// `Crescent::new(0.4, 0.6, 0.55, 40, mat)`
    pub fn new(
        outer_r: f64,
        inner_r: f64,
        offset_x: f64,
        segments: usize,
        mat: Arc<dyn Material>,
    ) -> Self {
        let mut tris = HittableList::new();
        let thick = 0.02;

        // ----- compute intersection of the two circles -----
        let denom = 2.0 * offset_x;
        if denom.abs() < 1e-10 {
            return Crescent {
                triangles: HittableList::new(),
            };
        }
        let ix = (outer_r * outer_r - inner_r * inner_r + offset_x * offset_x) / denom;
        let disc = outer_r * outer_r - ix * ix;
        if disc <= 0.0 {
            return Crescent {
                triangles: HittableList::new(),
            };
        }
        let iy = disc.sqrt();
        let theta = iy.atan2(ix); // half-angle subtended by the crescent

        let n = segments;

        // ----- generate boundary vertices (before rotation) -----
        // Outer arc: -θ → +θ   (N+1 points)
        // Inner arc: +θ → -θ   (N+1 points, closes the loop)
        let mut outer: Vec<Point3> = Vec::with_capacity(n + 1);
        let mut inner: Vec<Point3> = Vec::with_capacity(n + 1);

        for i in 0..=n {
            let t = i as f64 / n as f64;
            let a = -theta + t * 2.0 * theta;
            outer.push(Point3::new(outer_r * a.cos(), outer_r * a.sin(), 0.0));
        }
        for i in 0..=n {
            let t = i as f64 / n as f64;
            let a = theta - t * 2.0 * theta;
            inner.push(Point3::new(
                offset_x + inner_r * a.cos(),
                inner_r * a.sin(),
                0.0,
            ));
        }

        // ----- rotate 90° CCW around Z so the outer-arc midpoint faces +Y (top) -----
        // (x, y) -> (-y, x)
        let rotate = |p: &Point3| -> Point3 { Point3::new(-p.y, p.x, p.z) };

        let outer: Vec<Point3> = outer.iter().map(rotate).collect();
        let inner: Vec<Point3> = inner.iter().map(rotate).collect();

        // ----- triangle strip between outer and inner arcs -----
        let uv_scale = 1.0 / (2.0 * outer_r);

        for i in 0..n {
            let o0 = &outer[i];
            let o1 = &outer[i + 1];
            let i0 = &inner[i];
            let i1 = &inner[i + 1];

            // front face (+Z) — two triangles per quad
            Self::emit_tri(&mut tris, o0, o1, i0, thick, uv_scale, mat.clone());
            Self::emit_tri(&mut tris, o1, i1, i0, thick, uv_scale, mat.clone());

            // back face (-Z) — reversed winding
            Self::emit_tri(&mut tris, o0, i0, o1, -thick, uv_scale, mat.clone());
            Self::emit_tri(&mut tris, o1, i0, i1, -thick, uv_scale, mat.clone());
        }

        Crescent { triangles: tris }
    }

    fn emit_tri(
        tris: &mut HittableList,
        a: &Point3,
        b: &Point3,
        c: &Point3,
        z: f64,
        uv_scale: f64,
        mat: Arc<dyn Material>,
    ) {
        let az = Point3::new(a.x, a.y, z);
        let bz = Point3::new(b.x, b.y, z);
        let cz = Point3::new(c.x, c.y, z);
        tris.add(Arc::new(Triangle::new(
            &az,
            &bz,
            &cz,
            (0.5 + a.x * uv_scale, 0.5 + a.y * uv_scale),
            (0.5 + b.x * uv_scale, 0.5 + b.y * uv_scale),
            (0.5 + c.x * uv_scale, 0.5 + c.y * uv_scale),
            mat,
        )));
    }
}

impl Hittable for Crescent {
    fn hit(&self, r: &Ray, ray_t: &mut Interval, rec: &mut HitRecord) -> bool {
        self.triangles.hit(r, ray_t, rec)
    }

    fn bounding_box(&self) -> Aabb {
        self.triangles.bounding_box()
    }
}
