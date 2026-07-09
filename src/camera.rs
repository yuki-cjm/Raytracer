use crate::color::{Color, get_color};
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, degrees_to_radians, random_double};
use crate::vec3::{Point3, Vec3};

use console::style;
use image::{ImageBuffer, RgbImage};
use indicatif::ProgressBar;

#[allow(dead_code)]
pub struct Camera {
    // Image
    pub aspect_ratio: f64,        // Ratio of image width over height
    pub image_height: u32,        // Rendered image height in pixel count
    pub image_width: u32,         // Rendered image width in pixel count
    pub samples_per_pixel: u32,   // Count of random samples for each pixel
    pub max_depth: i32,           // Maximum number of ray bounces into scene
    pub pixel_samples_scale: f64, // Color scale factor for a sum of pixel samples

    // Camera
    pub center: Point3,       // Camera center position
    pub focal_length: f64,    // Distance from camera center to viewport
    pub viewport_height: f64, // Height of the image viewport
    pub viewport_width: f64,  // Width of the image viewport
    pub pixel_delta_u: Vec3,  // Horizontal offset between pixels
    pub pixel_delta_v: Vec3,  // Vertical offset between pixels
    pub pixel00_loc: Point3,  // Location of the upper left pixel
    pub vfov: f64,            // Vertical view angle (field of view)
}

impl Camera {
    pub fn new(
        aspect_ratio: f64,
        image_width: u32,
        samples_per_pixel: u32,
        max_depth: i32,
        vfov: f64,
    ) -> Self {
        let image_height = ((image_width as f64) / aspect_ratio) as u32;
        let image_height = image_height.max(1);

        let pixel_samples_scale = 1.0 / samples_per_pixel as f64;

        let center = Point3::new(0.0, 0.0, 0.0);

        // Determine viewport dimensions.
        let focal_length = 1.0;
        let theta = degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        let viewport_height = 2.0 * h * focal_length;
        let viewport_width = viewport_height * (image_width as f64 / image_height as f64);

        // Calculate the vectors across the horizontal and down the vertical viewport edges.
        let viewport_u = Vec3::new(viewport_width, 0.0, 0.0);
        let viewport_v = Vec3::new(0.0, -viewport_height, 0.0);

        // Calculate the horizontal and vertical delta vectors from pixel to pixel.
        let pixel_delta_u = viewport_u / image_width as f64;
        let pixel_delta_v = viewport_v / image_height as f64;

        // Calculate the location of the upper left pixel.
        let viewport_upper_left =
            center - Vec3::new(0.0, 0.0, focal_length) - viewport_u / 2.0 - viewport_v / 2.0;
        let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

        Self {
            aspect_ratio,
            image_width,
            samples_per_pixel,
            max_depth,
            image_height,
            pixel_samples_scale,
            center,
            focal_length,
            viewport_height,
            viewport_width,
            pixel_delta_u,
            pixel_delta_v,
            pixel00_loc,
            vfov,
        }
    }

    pub fn render(&self, world: &dyn Hittable) {
        // output
        let path = std::path::Path::new("output/book1/image1.png");
        let prefix = path.parent().unwrap();
        std::fs::create_dir_all(prefix).expect("Cannot create all the parents");

        let mut img: RgbImage = ImageBuffer::new(self.image_width, self.image_height);

        let progress = if option_env!("CI").unwrap_or_default() == "true" {
            ProgressBar::hidden()
        } else {
            ProgressBar::new((self.image_height * self.image_width) as u64)
        };

        for j in 0..self.image_height {
            for i in 0..self.image_width {
                let pixel = img.get_pixel_mut(i, j);
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);
                for _sample in 0..self.samples_per_pixel {
                    let r = self.get_ray(i, j);
                    pixel_color += Self::ray_color(&r, self.max_depth, world);
                }
                pixel_color *= self.pixel_samples_scale;
                *pixel = image::Rgb(get_color(&pixel_color));
            }
            progress.inc(1);
        }
        progress.finish();

        println!(
            "Output image as \"{}\"",
            style(path.to_str().unwrap()).yellow()
        );
        img.save(path).expect("Cannot save the image to the file");
    }

    fn get_ray(&self, i: u32, j: u32) -> Ray {
        // Construct a camera ray originating from the origin and directed at randomly sampled
        // point around the pixel location i, j.

        let offset = Self::sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x) * self.pixel_delta_u)
            + ((j as f64 + offset.y) * self.pixel_delta_v);

        let ray_origin = self.center;
        let ray_direction = pixel_sample - ray_origin;

        Ray::new(&ray_origin, &ray_direction)
    }

    fn sample_square() -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn ray_color(r: &Ray, depth: i32, world: &dyn Hittable) -> Color {
        // If we've exceeded the ray bounce limit, no more light is gathered.
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = HitRecord::default();

        if world.hit(r, &Interval::new(0.001, INFINITY), &mut rec) {
            let mut scattered = Ray::new(&Point3::new(0.0, 0.0, 0.0), &Vec3::new(0.0, 0.0, 0.0));
            let mut attenuation = Color::new(0.0, 0.0, 0.0);
            if rec.mat.scatter(r, &rec, &mut attenuation, &mut scattered) {
                return attenuation * Self::ray_color(&scattered, depth - 1, world);
            } else {
                return Color::new(0.0, 0.0, 0.0);
            }
        }

        let unit_direction = r.dir.unit_vector();
        let a = 0.5 * (unit_direction.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}
