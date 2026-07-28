mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod disk;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod obj_loader;
mod onb;
mod pdf;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod star;
mod texture;
mod triangle;
mod vec3;

use std::collections::HashMap;
use std::sync::Arc;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::disk::Disk;
use crate::hittable::{Hittable, RotateY, RotateZ, Scale, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, EmptyMaterial, Lambertian, Material};
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::star::Star;
use crate::texture::{FlipTexture, ImageTexture};
use crate::vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // ============================================================
    // Room (expanded: x [-24,24], y [0,20], z depth 13)
    let tex_src = Arc::new(ImageTexture::new("星空.jpeg"));
    let mat_back: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(FlipTexture::new(tex_src.clone(), false, false))));
    let mat_ceil: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(FlipTexture::new(tex_src.clone(), false, true))));
    let mat_left: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(FlipTexture::new(tex_src.clone(), true, false))));
    let mat_right: Arc<dyn Material> = Arc::new(DiffuseLight::new(Arc::new(FlipTexture::new(tex_src.clone(), true, false))));
    // Back wall (no flip)
    world.add(Arc::new(Quad::new(&Point3::new(-24.,0.,-10.),&Vec3::new(48.,0.,0.),&Vec3::new(0.,20.,0.),mat_back.clone())));
    lights.add(Arc::new(Quad::new(&Point3::new(-24.,0.,-10.),&Vec3::new(48.,0.,0.),&Vec3::new(0.,20.,0.),Arc::new(EmptyMaterial))));
    // Ceiling (flip V)
    world.add(Arc::new(Quad::new(&Point3::new(-24.,20.,-10.),&Vec3::new(48.,0.,0.),&Vec3::new(0.,0.,13.),mat_ceil.clone())));
    lights.add(Arc::new(Quad::new(&Point3::new(-24.,20.,-10.),&Vec3::new(48.,0.,0.),&Vec3::new(0.,0.,13.),Arc::new(EmptyMaterial))));
    // Left wall (flip U)
    world.add(Arc::new(Quad::new(&Point3::new(-24.,0.,3.),&Vec3::new(0.,0.,-13.),&Vec3::new(0.,20.,0.),mat_left.clone())));
    lights.add(Arc::new(Quad::new(&Point3::new(-24.,0.,3.),&Vec3::new(0.,0.,-13.),&Vec3::new(0.,20.,0.),Arc::new(EmptyMaterial))));
    // Right wall
    world.add(Arc::new(Quad::new(&Point3::new(24.,0.,-10.),&Vec3::new(0.,0.,13.),&Vec3::new(0.,20.,0.),mat_right.clone())));
    lights.add(Arc::new(Quad::new(&Point3::new(24.,0.,-10.),&Vec3::new(0.,0.,13.),&Vec3::new(0.,20.,0.),Arc::new(EmptyMaterial))));
    // Ground
    let ground_mat: Arc<dyn Material> = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.55, 0.75)));
    world.add(Arc::new(Quad::new(&Point3::new(-24.,0.,3.),&Vec3::new(48.,0.,0.),&Vec3::new(0.,0.,-13.),ground_mat)));

    // ---- Shared materials ----
    let star_glow: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(&Color::new(30.0, 24.0, 7.0)));
    let glass_mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let string_mat: Arc<dyn Material> = Arc::new(DiffuseLight::from_color(&Color::new(0.6, 0.6, 0.65)));
    let empty_mat: Arc<dyn Material> = Arc::new(EmptyMaterial);

    // ---- Hanging stars ----
    let string_top_y = 7.8;
    let star_configs = [
        (-4.0, -1.5, 18.0, 4.5),
        (-2.5, -2.5, -12.0, 5.5),
        (-1.0, -3.0, 10.0, 5.0),
        (1.0, -2.0, -8.0, 6.0),
        (2.5, -1.0, 15.0, 4.8),
        (3.0, -3.0, -15.0, 5.5),
    ];

    for (x, z, tilt, hang_y) in &star_configs {
        let string_w = 0.012;
        let string_h = string_top_y - hang_y;
        world.add(Arc::new(Quad::new(
            &Point3::new(x - string_w, *hang_y, *z),
            &Vec3::new(2.0 * string_w, 0.0, 0.0),
            &Vec3::new(0.0, string_h, 0.0),
            string_mat.clone(),
        )));

        let star = Arc::new(Star::new(0.45, 0.17, star_glow.clone()));
        let star = Arc::new(RotateZ::new(star, *tilt));
        let star = Arc::new(Translate::new(star, &Vec3::new(*x, *hang_y, *z)));
        world.add(star.clone());

        let star_light = Arc::new(Star::new(0.45, 0.17, empty_mat.clone()));
        let star_light = Arc::new(RotateZ::new(star_light, *tilt));
        let star_light = Arc::new(Translate::new(star_light, &Vec3::new(*x, *hang_y, *z)));
        lights.add(star_light);
    }

    // ---- Glass spheres ----
    let glass_configs: [(f64, f64, f64); 4] = [
        (-3.5, -2.0, 5.0),
        (-2.0, -3.0, 4.5),
        (0.5, -3.0, 5.5),
        (3.0, -2.0, 5.2),
    ];

    for (x, z, hang_y) in &glass_configs {
        let string_w = 0.012;
        let string_h = string_top_y - hang_y;
        world.add(Arc::new(Quad::new(
            &Point3::new(x - string_w, *hang_y, *z),
            &Vec3::new(2.0 * string_w, 0.0, 0.0),
            &Vec3::new(0.0, string_h, 0.0),
            string_mat.clone(),
        )));

        let glass = Arc::new(Sphere::new_stationary(
            &Point3::new(*x, *hang_y - 0.35, *z),
            0.35,
            glass_mat.clone(),
        ));
        world.add(glass);
    }

    // ============================================================
    // Moon
    // ============================================================
    let moon_tex = Arc::new(ImageTexture::new("moon.png"));
    let moon_mat: Arc<dyn Material> = Arc::new(DiffuseLight::new(moon_tex));
    let moon_radius = 1.8;
    let moon_center = Point3::new(3.5, 7.0, -9.9);
    // uv_r < 0.5 to avoid sampling the background outside the moon in the texture
    let moon_disk = Arc::new(Disk::new_with_uv(moon_radius, 60, moon_mat, 0.5, 0.5, 0.44));
    // Disk in XY plane, back wall is at z=-10, camera looks toward -Z → no rotation needed
    let moon_disk = Arc::new(Translate::new(moon_disk, &moon_center));
    world.add(moon_disk.clone());
    let moon_light = Arc::new(Disk::new(moon_radius, 60, empty_mat.clone()));
    lights.add(Arc::new(Translate::new(moon_light, &moon_center)));

    // ============================================================
    // Statue
    // ============================================================
    let statue_mat: Arc<dyn Material> = Arc::new(Lambertian::from_color(
        &Color::new(0.35, 0.35, 0.38)));
    let mut mat_map: HashMap<String, Arc<dyn Material>> = HashMap::new();
    for name in &[
        "Material_#24",
        "Material_#26",
        "Material_#27",
        "Material_#28",
        "Material_#29",
        "Material_#30",
        "Material_#31",
    ] {
        mat_map.insert(name.to_string(), statue_mat.clone());
    }

    let statue = obj_loader::load_obj("models/雕像.obj", &mat_map);
    let sb = statue.bounding_box();
    let statue_height = sb.y.max - sb.y.min;

    // Scale statue to a camera-friendly size.
    let statue_scale = 4.5 / statue_height;
    let statue = Arc::new(Scale::new(Arc::new(statue), statue_scale));
    let statue_x = -2.5;
    let statue_z = 1.5;
    let lift = -sb.y.min * statue_scale;
    let moon_pos = Point3::new(3.5, 7.0, -9.9);
    let face_dir = moon_pos - Point3::new(statue_x, 0.0, statue_z);
    let yaw = face_dir.x.atan2(face_dir.z).to_degrees() - 10.0;

    let statue = Arc::new(RotateY::new(statue, yaw));
    let statue = Arc::new(Translate::new(statue, &Vec3::new(statue_x, lift, statue_z)));
    world.add(statue);

    // ============================================================
    // Flowers
    // ============================================================
    let flower_tex = Arc::new(ImageTexture::new("Property_Prop_KhaenriahFlower_01_Diffuse.png"));
    let flower_mat: Arc<dyn Material> = Arc::new(Lambertian::new(flower_tex));
    let mut flower_map: HashMap<String, Arc<dyn Material>> = HashMap::new();
    flower_map.insert("Property_Prop_KhaenriahFlower_01".to_string(), flower_mat);
    let flower_base = obj_loader::load_obj("models/未有之梦.obj", &flower_map);
    let fb = flower_base.bounding_box();

    // Sparse flowers in visible ground area (z ∈ [-9.9, -6.3])
    let flowers: [(f64, f64, f64, f64, f64); 21] = [
        (-5.5, -7.0, 2.3, 10.0, -5.0),
        (-3.5, -7.0, 2.5, 100.0, 4.0),
        (-1.5, -7.0, 2.2, 180.0, -3.0),
        (0.5, -7.0, 2.6, 260.0, 6.0),
        (2.5, -7.0, 2.1, 330.0, -4.0),
        (4.5, -7.0, 2.4, 70.0, 5.0),
        (-3.0, -8.5, 2.5, 130.0, 5.0),
        (0.0, -8.5, 2.1, 205.0, -5.0),
        (3.0, -8.5, 2.4, 285.0, 3.0),
        (-1.0, -9.5, 2.2, 160.0, 4.0),
        (2.0, -9.5, 2.5, 250.0, -3.0),
        (5.0, -9.5, 2.1, 320.0, 6.0),
        (0.0, -9.9, 2.3, 195.0, -5.0),
        (2.0, -6.5, 2.5, 140.0, 4.0),
        (-2.0, -8.0, 2.2, 50.0, 7.0),
        (6.5, -7.0, 2.1, 280.0, 6.0),
        (-7.0, -8.0, 2.1, 275.0, 4.0),
        (-9.0, -7.5, 2.4, 355.0, -5.0),
        (-8.5, -9.5, 2.3, 55.0, 3.0),
        (8.5, -8.0, 2.2, 260.0, -5.0),
        (7.5, -9.5, 2.1, 40.0, -7.0),
    ];

    for (x, z, s, ry, rz) in &flowers {
        let f = Arc::new(Scale::new(Arc::new(flower_base.clone()), *s));
        let f = Arc::new(RotateZ::new(f, *rz));
        let f = Arc::new(RotateY::new(f, *ry));
        // 0.3 puts lowest petal ~0.005*s above ground — barely floating, no clipping
        let lift = -fb.y.min * s * 0.3;
        let f = Arc::new(Translate::new(f, &Vec3::new(*x, lift, *z)));
        world.add(f);
    }

    // ======================================================
    // Dense flower field: random scatter in visible ground trapezoid
    // Bottom edge z=-6.33 x∈[-7.14,7.14]; at z=-10 x∈[-10.2,10.2]
    // x_max(z) = 7.14 - 0.828*(z + 6.33)
    // ======================================================
    {
        let mut rng = rand::thread_rng();
        use rand::Rng;
        let fb_clone = Arc::new(flower_base.clone());
        let z_near = -6.33;
        let z_far = -9.9;
        let count = 1100;
        for _ in 0..count {
            let z: f64 = rng.gen_range(z_far..z_near);
            let x_max = 7.14 - 0.828 * (z - z_near);
            let x: f64 = rng.gen_range(-x_max..x_max);
            let s: f64 = rng.gen_range(1.8..2.8);
            let ry: f64 = rng.gen_range(0.0..360.0);
            let rz: f64 = rng.gen_range(-8.0..8.0);
            let f = Arc::new(Scale::new(fb_clone.clone(), s));
            let f = Arc::new(RotateZ::new(f, rz));
            let f = Arc::new(RotateY::new(f, ry));
            let lift = -fb.y.min * s * 0.3;
            let f = Arc::new(Translate::new(f, &Vec3::new(x, lift, z)));
            world.add(f);
        }
    }

    // ============================================================
    // Fireflies
    // ============================================================
    let firefly_glow: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(&Color::new(16.0, 18.0, 6.0)));
    // (x, z, y, radius) — varied sizes & heights for depth
    let fireflies = [
        // === Floor layer (y=0.02-0.08, tiny, below petals) ===
        (-4.0, -2.5, 0.05, 0.02),
        (-1.5, -0.5, 0.03, 0.015),
        (0.5, -2.0, 0.06, 0.025),
        (3.0, -1.0, 0.04, 0.02),
        (6.0, -2.5, 0.05, 0.015),
        (-8.0, -3.5, 0.06, 0.02),
        (-12.0, -5.0, 0.04, 0.025),
        (-6.5, -6.5, 0.05, 0.015),
        (-3.5, -7.0, 0.07, 0.02),
        (2.5, -6.0, 0.04, 0.025),
        (5.5, -5.5, 0.06, 0.02),
        (8.0, -4.0, 0.05, 0.015),
        (10.5, -7.0, 0.07, 0.02),
        (-15.0, -7.5, 0.04, 0.025),
        (-9.5, -8.5, 0.06, 0.02),
        (0.0, -9.0, 0.05, 0.015),
        (6.5, -8.5, 0.04, 0.02),
        (12.0, -8.0, 0.06, 0.025),
        (-18.0, -9.0, 0.05, 0.02),
        (16.0, -9.5, 0.07, 0.015),
        // === Petal-layer (y=0.15-0.35, medium, among flowers) ===
        (-5.0, -1.5, 0.20, 0.04),
        (-3.0, 0.0, 0.25, 0.035),
        (-1.0, -2.0, 0.18, 0.05),
        (1.0, -1.0, 0.30, 0.03),
        (3.0, -2.5, 0.22, 0.045),
        (5.0, -1.5, 0.16, 0.055),
        (-7.5, -1.0, 0.28, 0.04),
        (-2.0, 2.0, 0.20, 0.035),
        (0.0, 1.5, 0.32, 0.03),
        (2.0, 0.5, 0.18, 0.05),
        (7.5, -1.0, 0.24, 0.04),
        (-9.0, 2.5, 0.22, 0.045),
        (9.0, 2.0, 0.26, 0.035),
        // Dense flower field area — offset from flower positions
        (-0.3, -2.7, 0.19, 0.04),
        (0.7, -0.2, 0.22, 0.035),
        (1.3, -2.3, 0.17, 0.05),
        (2.3, -0.3, 0.25, 0.04),
        (3.3, -2.8, 0.20, 0.045),
        (4.3, -0.8, 0.28, 0.035),
        (-0.8, -3.7, 0.23, 0.04),
        (0.2, -4.3, 0.18, 0.05),
        (1.2, -3.8, 0.26, 0.03),
        (2.2, -4.3, 0.21, 0.045),
        (3.2, -3.3, 0.30, 0.035),
        (4.2, -3.8, 0.17, 0.04),
        (-0.6, -1.3, 0.24, 0.04),
        (0.9, -0.8, 0.16, 0.05),
        (1.7, -1.3, 0.29, 0.035),
        (2.8, -0.8, 0.20, 0.045),
        (3.8, -1.3, 0.23, 0.04),
        // === Left side (x=-20..-6, z=-2..-9) ===
        (-10.0, -3.0, 0.18, 0.05),
        (-8.5, -4.5, 0.25, 0.04),
        (-12.5, -4.0, 0.15, 0.06),
        (-10.5, -5.5, 0.28, 0.035),
        (-7.0, -5.5, 0.22, 0.045),
        (-13.5, -6.0, 0.17, 0.055),
        (-9.0, -7.0, 0.30, 0.04),
        (-11.5, -7.5, 0.20, 0.035),
        (-14.5, -6.5, 0.26, 0.045),
        (-8.0, -8.5, 0.18, 0.05),
        (-12.0, -8.0, 0.24, 0.04),
        (-16.0, -8.0, 0.16, 0.055),
        (-10.0, -9.0, 0.28, 0.035),
        (-14.0, -9.5, 0.22, 0.045),
        (-17.5, -9.5, 0.20, 0.04),
        // === Center zone (x=-5..5, z=-3..-9, not in flower field) ===
        (-4.5, -3.5, 0.30, 0.035),
        (-2.5, -3.0, 0.18, 0.05),
        (0.5, -4.5, 0.25, 0.04),
        (2.5, -4.5, 0.16, 0.055),
        (4.5, -4.5, 0.28, 0.04),
        (-5.0, -5.5, 0.20, 0.045),
        (-3.0, -5.0, 0.30, 0.035),
        (1.0, -6.0, 0.18, 0.05),
        (3.0, -6.5, 0.22, 0.04),
        (5.0, -6.0, 0.26, 0.045),
        (-4.5, -8.0, 0.15, 0.055),
        (-1.0, -8.5, 0.28, 0.04),
        (1.0, -9.0, 0.20, 0.035),
        (3.0, -9.5, 0.24, 0.045),
        (-3.5, -9.5, 0.16, 0.05),
        // === Right side (x=6..20, z=-2..-9) ===
        (7.0, -3.0, 0.22, 0.04),
        (9.5, -3.5, 0.16, 0.055),
        (11.5, -4.0, 0.28, 0.035),
        (8.5, -5.5, 0.20, 0.045),
        (10.0, -5.0, 0.30, 0.04),
        (6.5, -6.5, 0.18, 0.05),
        (12.5, -6.0, 0.25, 0.04),
        (9.0, -7.5, 0.16, 0.055),
        (7.5, -7.0, 0.28, 0.035),
        (14.0, -7.0, 0.22, 0.045),
        (8.0, -8.5, 0.18, 0.05),
        (11.0, -8.0, 0.26, 0.04),
        (15.5, -8.0, 0.20, 0.04),
        (10.5, -9.0, 0.24, 0.045),
        (6.0, -9.0, 0.16, 0.055),
        (12.5, -9.5, 0.28, 0.035),
        (14.5, -9.0, 0.22, 0.04),
        (16.5, -9.5, 0.18, 0.05),
        // === Low air (y=0.4-0.9, varied sizes) ===
        (-4.5, -2.5, 0.55, 0.03),
        (-2.5, -3.5, 0.70, 0.05),
        (0.5, -3.0, 0.45, 0.025),
        (2.5, -3.5, 0.65, 0.06),
        (-3.5, -4.5, 0.50, 0.035),
        (1.5, -5.0, 0.80, 0.04),
        (3.5, -4.5, 0.40, 0.02),
        (-1.5, -5.5, 0.60, 0.055),
        (-5.0, -6.0, 0.75, 0.03),
        (-8.5, -4.0, 0.50, 0.045),
        (8.5, -4.5, 0.55, 0.035),
        (-10.5, -6.5, 0.65, 0.025),
        (11.5, -6.0, 0.45, 0.05),
        (-14.5, -5.0, 0.70, 0.03),
        (14.5, -6.5, 0.60, 0.04),
        (-7.0, -9.0, 0.50, 0.045),
        (7.0, -9.0, 0.55, 0.035),
        (-16.0, -7.0, 0.80, 0.03),
        (16.0, -7.5, 0.75, 0.04),
        // === Mid air (y=1.0-2.0, larger, sparse) ===
        (-3.0, -2.0, 1.3, 0.06),
        (1.0, -3.0, 1.5, 0.07),
        (-5.0, -4.0, 1.2, 0.05),
        (3.0, -4.0, 1.8, 0.08),
        (-2.0, -5.0, 1.1, 0.06),
        (0.0, -6.0, 1.6, 0.07),
        (-4.0, -7.0, 1.4, 0.055),
        (2.0, -7.0, 1.7, 0.065),
        (-8.0, -3.0, 1.2, 0.06),
        (8.0, -3.5, 1.5, 0.07),
        (-12.0, -5.5, 1.8, 0.05),
        (12.0, -5.0, 1.3, 0.08),
        (-10.0, -8.0, 1.6, 0.06),
        (10.0, -8.0, 1.1, 0.07),
        // === High air (y=2.0-3.5, biggest, few) ===
        (-2.0, -1.5, 2.5, 0.07),
        (2.0, -2.5, 2.8, 0.08),
        (-4.0, -3.5, 2.2, 0.065),
        (1.0, -5.5, 3.0, 0.07),
        (-6.0, -6.5, 2.6, 0.06),
        (5.0, -7.5, 2.3, 0.075),
        (-10.0, -4.5, 2.8, 0.065),
        (10.0, -5.5, 3.2, 0.07),
        (-14.0, -7.0, 2.5, 0.06),
        (14.0, -7.0, 2.7, 0.075),
    ];
    for (x, z, y, r) in &fireflies {
        let ff = Arc::new(Sphere::new_stationary(
            &Point3::new(*x, *y, *z),
            *r,
            firefly_glow.clone(),
        ));
        world.add(ff.clone());
        let ff_l = Arc::new(Sphere::new_stationary(
            &Point3::new(*x, *y, *z),
            *r,
            empty_mat.clone(),
        ));
        lights.add(ff_l);
    }

    // ============================================================
    // Camera
    // ============================================================
    let lookfrom = Point3::new(0.0, 3.5, 2.5);
    let lookat = Point3::new(0.0, 4.0, -6.0);
    let focus_dist = (lookfrom - lookat).length();
    let cam = Camera::new(
        16.0 / 9.0,
        2560,
        100,
        20,
        &Color::new(0.80, 0.80, 1.0),
        50.0,
        &lookfrom,
        &lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        focus_dist,
    );

    let world_bvh = BvhNode::new(&mut world);
    cam.render(&world_bvh, &lights);
}
