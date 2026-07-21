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
mod texture;
mod triangle;
mod vec3;

use std::collections::HashMap;
use std::sync::Arc;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, RotateY, Scale, Translate};
use crate::hittable_list::HittableList;
use crate::material::{DiffuseLight, EmptyMaterial, Lambertian, Material};
use crate::quad::Quad;
use crate::texture::ImageTexture;
use crate::vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::new();

    // Build material map: all materials use the single texture
    let mut mat_map: HashMap<String, Arc<dyn Material>> = HashMap::new();

    let tex = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
        "textures/茜特菈琳.png",
    ))));

    for name in &["Citlali_Ghost01", "Citlali_Ghost02", "Citlali_Ghost03"] {
        mat_map.insert(name.to_string(), tex.clone());
    }

    let model = obj_loader::load_obj("models/茜特拉琳.obj", &mat_map);

    // Rotate 90° around Y, then scale and place centered
    let model = Arc::new(RotateY::new(Arc::new(model), 180.0));
    let model = Arc::new(Scale::new(model, 200.0));
    let model = Arc::new(Translate::new(model, &Vec3::new(278.0, 0.0, 278.0)));
    world.add(model);

    let bbox = world.bounding_box();
    eprintln!(
        "World bbox: x=[{:.2}, {:.2}], y=[{:.2}, {:.2}], z=[{:.2}, {:.2}]",
        bbox.x.min, bbox.x.max, bbox.y.min, bbox.y.max, bbox.z.min, bbox.z.max
    );

    let light_material = Arc::new(DiffuseLight::from_color(&Color::new(15.0, 15.0, 15.0)));
    world.add(Arc::new(Quad::new(
        &Point3::new(200.0, 554.0, 200.0),
        &Vec3::new(150.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 150.0),
        light_material.clone(),
    )));

    let empty_material = Arc::new(EmptyMaterial);
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        &Point3::new(200.0, 554.0, 200.0),
        &Vec3::new(150.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 150.0),
        empty_material.clone(),
    )));

    let cam = Camera::new(
        1.0,
        1200,
        50,
        50,
        &Color::new(0.7, 0.8, 1.0),
        28.0,
        &Point3::new(278.0, 120.0, -180.0),
        &Point3::new(278.0, 100.0, 278.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world, &lights);
}
