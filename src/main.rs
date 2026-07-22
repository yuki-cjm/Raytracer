mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
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
use crate::hittable::{Hittable, Scale, Translate};
use crate::hittable_list::HittableList;
use crate::material::{DiffuseLight, EmptyMaterial, Lambertian, Material};
use crate::quad::Quad;
use crate::texture::ImageTexture;
use crate::vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::new();
    let mut lights = HittableList::new();

    // ---- Ground: light blue ----
    let ground_mat: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(&Color::new(0.35, 0.55, 0.7)));
    world.add(Arc::new(Quad::new(
        &Point3::new(-3.0, 0.0, 3.0),
        &Vec3::new(6.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -6.0),
        ground_mat,
    )));

    // ---- Invisible overhead light (above camera frame) ----
    let light_mat = Arc::new(DiffuseLight::from_color(&Color::new(7.0, 6.5, 8.0)));
    // Small quad high above the flower, out of camera view
    world.add(Arc::new(Quad::new(
        &Point3::new(-0.5, 3.0, 0.3),
        &Vec3::new(1.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -0.6),
        light_mat,
    )));

    let empty_mat = Arc::new(EmptyMaterial);
    lights.add(Arc::new(Quad::new(
        &Point3::new(-0.5, 3.0, 0.3),
        &Vec3::new(1.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -0.6),
        empty_mat,
    )));

    // ---- Flower material with diffuse texture ----
    let flower_tex = Arc::new(ImageTexture::new(
        "Property_Prop_KhaenriahFlower_01_Diffuse.png",
    ));
    let flower_mat: Arc<dyn Material> = Arc::new(Lambertian::new(flower_tex));

    let mut mat_map: HashMap<String, Arc<dyn Material>> = HashMap::new();
    mat_map.insert("Property_Prop_KhaenriahFlower_01".to_string(), flower_mat);

    // ---- Load single flower ----
    let flower = obj_loader::load_obj("models/未有之梦.obj", &mat_map);
    let fb = flower.bounding_box();
    let flower = Arc::new(Scale::new(Arc::new(flower), 3.0));
    let lift = -fb.y.min * 3.0;
    let flower = Arc::new(Translate::new(flower, &Vec3::new(0.0, lift, 0.0)));
    world.add(flower);

    // ---- Camera ----
    let lookfrom = Point3::new(0.0, 1.5, 2.0);
    let lookat = Point3::new(0.0, 0.3, 0.0);
    let focus_dist = (lookfrom - lookat).length();
    let cam = Camera::new(
        1.0,
        1200,
        100,
        50,
        &Color::new(0.3, 0.3, 0.5),
        40.0,
        &lookfrom,
        &lookat,
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        focus_dist,
    );

    cam.render(&world, &lights);
}
