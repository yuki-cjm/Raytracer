mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Metal};
use crate::sphere::Sphere;
use crate::vec3::Point3;
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    let material_ground = Rc::new(Lambertian::new(&Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(Lambertian::new(&Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(Dielectric::new(1.00 / 1.33));
    let material_right = Rc::new(Metal::new(&Color::new(0.8, 0.6, 0.2), 1.0));

    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let cam = Camera::new(16.0 / 9.0, 400u32, 100, 50);
    cam.render(&world);
}
