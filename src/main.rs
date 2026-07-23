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

use crate::camera::Camera;
use crate::color::Color;
use crate::disk::Disk;
use crate::hittable::{Hittable, RotateY, RotateZ, Scale, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, EmptyMaterial, Lambertian, Material, Metal};
use crate::quad::Quad;
use crate::sphere::Sphere;
use crate::star::Star;
use crate::texture::ImageTexture;
use crate::vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // ============================================================
    // Room
    // ============================================================
    let wall_tex = Arc::new(ImageTexture::new("星空.jpeg"));
    let wall_mat: Arc<dyn Material> = Arc::new(DiffuseLight::new(wall_tex));
    world.add(Arc::new(Quad::new(
        &Point3::new(-12.0, 0.0, -10.0),
        &Vec3::new(24.0, 0.0, 0.0),
        &Vec3::new(0.0, 15.0, 0.0),
           wall_mat.clone(),
    )));
    lights.add(Arc::new(Quad::new(
        &Point3::new(-12.0, 0.0, -10.0),
        &Vec3::new(24.0, 0.0, 0.0),
        &Vec3::new(0.0, 15.0, 0.0),
        Arc::new(EmptyMaterial),
    )));

    // Ceiling
    world.add(Arc::new(Quad::new(&Point3::new(-12.0,15.0,3.0),&Vec3::new(24.0,0.0,0.0),&Vec3::new(0.0,0.0,-13.0),wall_mat.clone())));
    lights.add(Arc::new(Quad::new(&Point3::new(-12.0,15.0,3.0),&Vec3::new(24.0,0.0,0.0),&Vec3::new(0.0,0.0,-13.0),Arc::new(EmptyMaterial))));
    // Left wall
    world.add(Arc::new(Quad::new(&Point3::new(-12.0,0.0,3.0),&Vec3::new(0.0,0.0,-13.0),&Vec3::new(0.0,15.0,0.0),wall_mat.clone())));
    lights.add(Arc::new(Quad::new(&Point3::new(-12.0,0.0,3.0),&Vec3::new(0.0,0.0,-13.0),&Vec3::new(0.0,15.0,0.0),Arc::new(EmptyMaterial))));
    // Right wall
    world.add(Arc::new(Quad::new(&Point3::new(12.0,0.0,-10.0),&Vec3::new(0.0,0.0,13.0),&Vec3::new(0.0,15.0,0.0),wall_mat.clone())));
    lights.add(Arc::new(Quad::new(&Point3::new(12.0,0.0,-10.0),&Vec3::new(0.0,0.0,13.0),&Vec3::new(0.0,15.0,0.0),Arc::new(EmptyMaterial))));

    // Ground
    let ground_mat: Arc<dyn Material> = Arc::new(Metal::new(&Color::new(0.15, 0.25, 0.40), 0.05));
    world.add(Arc::new(Quad::new(
        &Point3::new(-12.0, 0.0, 3.0),
        &Vec3::new(24.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -13.0),
        ground_mat,
    )));

    // ---- Shared materials ----
    let star_glow: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(&Color::new(30.0, 24.0, 7.0)));
    let glass_mat: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));
    let string_mat: Arc<dyn Material> = Arc::new(Metal::new(&Color::new(0.75, 0.75, 0.78), 0.05));
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

    // Room-sized flower grid (x=-6..6, z=0..-8)
    let flowers: [(f64, f64, f64, f64, f64); 37] = [
        (-3.0, -1.5, 2.5, 110.0, -3.0),
        (-1.0, -1.0, 2.0, 190.0, 6.0),
        (1.0, -1.5, 2.3, 270.0, -5.0),
        (3.0, -1.0, 2.6, 350.0, 3.0),
        (-5.5, -3.0, 2.4, 80.0, -6.0),
        (-3.5, -3.0, 2.1, 150.0, 5.0),
        (-1.5, -3.0, 2.7, 230.0, -2.0),
        (0.5, -3.0, 2.2, 310.0, 7.0),
        (2.5, -3.0, 2.5, 40.0, -5.0),
        (4.5, -3.0, 2.0, 120.0, 4.0),
        (-4.0, -5.0, 2.6, 140.0, -4.0),
        (-2.0, -5.0, 2.1, 210.0, 6.0),
        (0.0, -5.0, 2.4, 290.0, -3.0),
        (2.0, -5.0, 2.2, 25.0, 5.0),
        (4.0, -5.0, 2.5, 85.0, -6.0),
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
        // Added to fill sparse areas
        (-1.5, -1.0, 2.4, 75.0, -6.0),
        (1.5, -1.5, 2.1, 220.0, 5.0),
        (-4.5, -6.0, 2.3, 310.0, -2.0),
        (2.0, -6.5, 2.5, 140.0, 4.0),
        (-2.0, -8.0, 2.2, 50.0, 7.0),
        // Right-side fill (x=5+ at z=-5 or deeper OK)
        (6.0, -3.0, 2.2, 130.0, 5.0),
        (5.5, -5.0, 2.5, 200.0, -4.0),
        (6.5, -7.0, 2.1, 280.0, 6.0),
        (5.0, -3.0, 2.3, 15.0, -7.0),
    ];

    for (x, z, s, ry, rz) in &flowers {
        let f = Arc::new(Scale::new(Arc::new(flower_base.clone()), *s));
        let f = Arc::new(RotateZ::new(f, *rz));
        let f = Arc::new(RotateY::new(f, *ry));
        let lift = -fb.y.min * s;
        let f = Arc::new(Translate::new(f, &Vec3::new(*x, lift, *z)));
        world.add(f);
    }

    // ============================================================
    // Fireflies
    // ============================================================
    let firefly_glow: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(&Color::new(16.0, 18.0, 6.0)));
    // (x, z, y, radius)
    let fireflies = [
        (-5.0, -1.0, 0.15, 0.05),
        (-3.0, 0.5, 0.25, 0.03),
        (-1.0, -1.5, 0.10, 0.06),
        (1.0, -0.5, 0.30, 0.04),
        (3.0, -2.0, 0.20, 0.05),
        (5.0, -1.0, 0.12, 0.06),
        (-4.0, -3.0, 0.28, 0.03),
        (-2.0, -2.5, 0.18, 0.05),
        (0.0, -3.5, 0.22, 0.04),
        (2.0, -3.0, 0.14, 0.06),
        (4.0, -4.0, 0.32, 0.04),
        (-5.0, -5.0, 0.08, 0.06),
        (-3.0, -4.5, 0.26, 0.05),
        (1.0, -5.0, 0.16, 0.03),
        (3.0, -5.5, 0.20, 0.06),
        (5.0, -5.0, 0.18, 0.04),
        (-6.0, -6.0, 0.30, 0.05),
        (0.0, -6.5, 0.12, 0.06),
        (2.0, -6.0, 0.22, 0.04),
        (4.0, -7.0, 0.15, 0.05),
        // Airborne
        (-4.0, -2.0, 1.2, 0.04),
        (-2.0, -3.0, 2.0, 0.05),
        (0.0, -2.5, 1.5, 0.03),
        (2.0, -3.5, 2.5, 0.06),
        (-3.0, -4.0, 1.8, 0.04),
        (1.0, -4.5, 3.0, 0.05),
        (3.0, -4.0, 2.2, 0.03),
        (-1.0, -5.0, 1.0, 0.05),
        (4.0, -5.5, 2.8, 0.04),
        (-5.0, -5.5, 1.6, 0.05),
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

    cam.render(&world, &lights);
}
