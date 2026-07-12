mod aabb;
mod bvh;
mod camera;
mod color;
mod hittable;
mod hittable_list;
mod interval;
mod material;
mod perlin;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use std::rc::Rc;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
#[allow(unused_imports)]
use crate::material::{Dielectric, Lambertian, Metal};
#[allow(unused_imports)]
use crate::rtweekend::{PI, random_double, random_range};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
#[allow(unused_imports)]
use crate::vec3::{Point3, Vec3};

fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(checker)),
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.8 {
                    // diffuse
                    let albedo = Color::random_vec3() * Color::random_vec3();
                    let sphere_material = Rc::new(Lambertian::from_color(&albedo));
                    let center2 = center + Vec3::new(0.0, random_range(0.0, 0.5), 0.0);
                    world.add(Rc::new(Sphere::new_moving(
                        &center,
                        &center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_vec3_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Rc::new(Metal::new(&albedo, fuzz));
                    world.add(Rc::new(Sphere::new_stationary(
                        &center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Rc::new(Dielectric::new(1.5));
                    world.add(Rc::new(Sphere::new_stationary(
                        &center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Rc::new(Dielectric::new(1.5));
    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Rc::new(Lambertian::from_color(&Color::new(0.4, 0.2, 0.1)));
    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Rc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world = HittableList::new_one(Rc::new(BvhNode::new(&mut world)));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.6,
        10.0,
    );

    cam.render(&world);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Rc::new(CheckerTexture::from_colors(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        Rc::new(Lambertian::new(checker.clone())),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn earth() {
    let earth_texture = Rc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Rc::new(Lambertian::new(earth_texture));
    let globe = Rc::new(Sphere::new_stationary(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        &Point3::new(0.0, 0.0, 12.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&HittableList::new_one(globe));
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let pertext = Rc::new(NoiseTexture::new());
    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Rc::new(Sphere::new_stationary(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        Rc::new(Lambertian::new(pertext.clone())),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn main() {
    let mode = 4;
    match mode {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        _ => unreachable!("invalid mode {}", mode),
    };
}
