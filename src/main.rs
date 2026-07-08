mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod ray;
mod rtweekend;
mod sphere;
mod vec3;

use crate::camera::Camera;
use crate::hittable_list::HittableList;
use crate::sphere::Sphere;
use crate::vec3::Point3;

fn main() {
    let mut world = HittableList::new();

    world.add(Box::new(Sphere::new(&Point3::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(
        &Point3::new(0.0, -100.5, -1.0),
        100.0,
    )));

    let cam = Camera::new(16.0 / 9.0, 400u32);
    cam.render(&world);
}
