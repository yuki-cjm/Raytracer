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
mod perlin;
mod quad;
mod ray;
mod rtw_stb_image;
mod rtweekend;
mod sphere;
mod texture;
mod vec3;

use std::sync::Arc;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::constant_medium::ConstantMedium;
use crate::hittable::{RotateY, Translate};
use crate::hittable_list::HittableList;
#[allow(unused_imports)]
use crate::material::{Dielectric, DiffuseLight, Lambertian, Metal};
use crate::quad::{Quad, box_shape};
use crate::rtweekend::{random_double, random_range};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, ImageTexture, NoiseTexture};
use crate::vec3::{Point3, Vec3};

fn bouncing_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(checker)),
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
                    let sphere_material = Arc::new(Lambertian::from_color(&albedo));
                    let center2 = center + Vec3::new(0.0, random_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        &center,
                        &center2,
                        0.2,
                        sphere_material,
                    )));
                } else if choose_mat < 0.95 {
                    // metal
                    let albedo = Color::random_vec3_range(0.5, 1.0);
                    let fuzz = random_range(0.0, 0.5);
                    let sphere_material = Arc::new(Metal::new(&albedo, fuzz));
                    world.add(Arc::new(Sphere::new_stationary(
                        &center,
                        0.2,
                        sphere_material,
                    )));
                } else {
                    // glass
                    let sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new_stationary(
                        &center,
                        0.2,
                        sphere_material,
                    )));
                }
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::from_color(&Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(&Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let world = HittableList::new_one(Arc::new(BvhNode::new(&mut world)));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.70, 0.80, 1.00),
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

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        &Color::new(0.2, 0.3, 0.1),
        &Color::new(0.9, 0.9, 0.9),
    ));

    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, -10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 10.0, 0.0),
        10.0,
        Arc::new(Lambertian::new(checker.clone())),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.70, 0.80, 1.00),
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
    let earth_texture = Arc::new(ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::new(earth_texture));
    let globe = Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 0.0, 0.0),
        2.0,
        earth_surface,
    ));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.70, 0.80, 1.00),
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

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.70, 0.80, 1.00),
        20.0,
        &Point3::new(13.0, 2.0, 3.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    // Materials
    let left_red = Arc::new(Lambertian::from_color(&Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::from_color(&Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::from_color(&Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::from_color(&Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::from_color(&Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Arc::new(Quad::new(
        &Point3::new(-3.0, -2.0, 5.0),
        &Vec3::new(0.0, 0.0, -4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        left_red.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, -2.0, 0.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 4.0, 0.0),
        back_green.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(3.0, -2.0, 1.0),
        &Vec3::new(0.0, 0.0, 4.0),
        &Vec3::new(0.0, 4.0, 0.0),
        right_blue.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, 3.0, 1.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 4.0),
        upper_orange.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(-2.0, -3.0, 5.0),
        &Vec3::new(4.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, -4.0),
        lower_teal.clone(),
    )));

    let cam = Camera::new(
        1.0,
        400,
        100,
        50,
        &Color::new(0.70, 0.80, 1.00),
        80.0,
        &Point3::new(0.0, 0.0, 9.0),
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn simple_light() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    let difflight = Arc::new(DiffuseLight::from_color(&Color::new(4.0, 4.0, 4.0)));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(3.0, 1.0, -2.0),
        &Vec3::new(2.0, 0.0, 0.0),
        &Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    let cam = Camera::new(
        16.0 / 9.0,
        400,
        100,
        50,
        &Color::new(0.0, 0.0, 0.0),
        20.0,
        &Point3::new(26.0, 3.0, 6.0),
        &Point3::new(0.0, 2.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn cornell_box() {
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

    let box1 = box_shape(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let box2 = box_shape(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white,
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    let cam = Camera::new(
        1.0,
        600,
        10,
        50,
        &Color::new(0.0, 0.0, 0.0),
        40.0,
        &Point3::new(278.0, 278.0, -800.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::from_color(&Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(&Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(&Color::new(7.0, 7.0, 7.0)));

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
        &Point3::new(113.0, 554.0, 127.0),
        &Vec3::new(330.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 305.0),
        light.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 555.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 0.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        &Point3::new(0.0, 0.0, 555.0),
        &Vec3::new(555.0, 0.0, 0.0),
        &Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let box1 = box_shape(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, &Vec3::new(265.0, 0.0, 295.0)));

    let box2 = box_shape(
        &Point3::new(0.0, 0.0, 0.0),
        &Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, &Vec3::new(130.0, 0.0, 65.0)));

    world.add(Arc::new(ConstantMedium::from_color(
        box1,
        0.01,
        &Color::new(0.0, 0.0, 0.0),
    )));
    world.add(Arc::new(ConstantMedium::from_color(
        box2,
        0.01,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let cam = Camera::new(
        1.0,
        600,
        200,
        50,
        &Color::new(0.0, 0.0, 0.0),
        40.0,
        &Point3::new(278.0, 278.0, -800.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn final_scene(image_width: u32, samples_per_pixel: u32, max_depth: i32) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::from_color(&Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(box_shape(
                &Point3::new(x0, y0, z0),
                &Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();

    world.add(Arc::new(BvhNode::new(&mut boxes1)));

    let light = Arc::new(DiffuseLight::from_color(&Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        &Point3::new(123.0, 554.0, 147.0),
        &Vec3::new(300.0, 0.0, 0.0),
        &Vec3::new(0.0, 0.0, 265.0),
        light.clone(),
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::from_color(&Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        &center1,
        &center2,
        50.0,
        sphere_material.clone(),
    )));

    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(&Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new_stationary(
        &Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());
    world.add(Arc::new(ConstantMedium::from_color(
        boundary.clone(),
        0.2,
        &Color::new(0.2, 0.4, 0.9),
    )));
    let boundary = Arc::new(Sphere::new_stationary(
        &Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Arc::new(ConstantMedium::from_color(
        boundary.clone(),
        0.0001,
        &Color::new(1.0, 1.0, 1.0),
    )));

    let emat = Arc::new(Lambertian::new(Arc::new(ImageTexture::new("earthmap.jpg"))));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(400.0, 200.0, 400.0),
        100.0,
        emat.clone(),
    )));
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Arc::new(Sphere::new_stationary(
        &Point3::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    let mut boxes2 = HittableList::new();
    let white = Arc::new(Lambertian::from_color(&Color::new(0.73, 0.73, 0.73)));
    for _ in 0..1000 {
        boxes2.add(Arc::new(Sphere::new_stationary(
            &Point3::random_vec3_range(0.0, 165.0),
            10.0,
            white.clone(),
        )));
    }

    world.add(Arc::new(Translate::new(
        Arc::new(RotateY::new(Arc::new(BvhNode::new(&mut boxes2)), 15.0)),
        &Vec3::new(-100.0, 270.0, 395.0),
    )));

    let cam = Camera::new(
        1.0,
        image_width,
        samples_per_pixel,
        max_depth,
        &Color::new(0.0, 0.0, 0.0),
        40.0,
        &Point3::new(478.0, 278.0, -600.0),
        &Point3::new(278.0, 278.0, 0.0),
        &Vec3::new(0.0, 1.0, 0.0),
        0.0,
        10.0,
    );

    cam.render(&world);
}

fn main() {
    let mode = 7;
    match mode {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 10000, 40),
        _ => final_scene(400, 250, 4),
    };
}
