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
#[allow(unused_imports)]
use crate::material::{Dielectric, Lambertian, Metal};
use crate::rtweekend::PI;
use crate::sphere::Sphere;
use crate::vec3::Point3;
use std::rc::Rc;

fn main() {
    let mut world = HittableList::new();

    let r = (PI / 4.0).cos();

    let material_left = Rc::new(Lambertian::new(&Color::new(0.0, 0.0, 1.0)));
    let material_right = Rc::new(Lambertian::new(&Color::new(1.0, 0.0, 0.0)));

    world.add(Box::new(Sphere::new(
        &Point3::new(-r, 0.0, -1.0),
        r,
        material_left,
    )));
    world.add(Box::new(Sphere::new(
        &Point3::new(r, 0.0, -1.0),
        r,
        material_right,
    )));

    let cam = Camera::new(16.0 / 9.0, 400, 100, 50, 90.0);
    cam.render(&world);
}
