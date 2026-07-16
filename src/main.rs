mod aabb;
mod bvh;
mod camera;
mod color;
mod constant_medium;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod onb;
mod pdf;
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use std::sync::Arc;

#[allow(unused_imports)]
use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
#[allow(unused_imports)]
use crate::constant_medium::ConstantMedium;
use crate::hittable::{RotateY, Translate};
use crate::hittable_list::HittableList;
#[allow(unused_imports)]
use crate::material::{Dielectric, DiffuseLight, EmptyMaterial, Lambertian, Metal};
use crate::quad::{Quad, box_shape};
#[allow(unused_imports)]
use crate::rtweekend::{random_double, random_range};
#[allow(unused_imports)]
use crate::sphere::Sphere;
#[allow(unused_imports)]
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Point3, Vec3};

fn main() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(&Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        &Point3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(555.0, 555.0, 555.0),
        &Vec3::new(-555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 555.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Box
    let box1 = box_shape(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    // Glass Sphere
    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    // Light Sources
    let empty_material = Arc::new(EmptyMaterial);
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        &Point3::new(343.0, 554.0, 332.0),
        &Vec3::new(-130.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -105.0),
        empty_material.clone(),
    )));
    lights.add(Arc::new(Sphere::new_stationary(
        &Point3::new(190.0, 90.0, 190.0),
        90.0,
        empty_material.clone(),
    )));

    let cam = Camera::new(
        1.0,
        600,
        1000,
        50,
        &Color::new(0.0, 0.0, 0.0),
        40.0,
        &Point3::new(278.0, 278.0, -800.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world, &lights);
}
